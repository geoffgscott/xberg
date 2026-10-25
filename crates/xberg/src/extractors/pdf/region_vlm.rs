//! Per-region VLM extraction for PDF layout regions.
//!
//! When `vlm_fallback != Disabled` and layout detection has identified
//! `Picture` (figure), `Table` (dense table), or `Other` (unclassified complex
//! layout) regions, this module crops those regions from the page raster, calls
//! the VLM via [`crate::llm::region_extractor`], and splices the resulting
//! markdown back into the assembled document at the region's original position.
//! A table or other region's answer replaces the native tables and body text
//! inside it; a figure's description is added beside it.
//!
//! Only compiled when `liter-llm` + `layout-detection` are both enabled
//! (not on Windows).

#![cfg(all(feature = "liter-llm", feature = "layout-detection"))]

use std::io::Cursor;

use image::{ExtendedColorType, ImageEncoder};

use crate::RegionKind;
use crate::core::config::LlmConfig;
use crate::layout::{DetectionResult, LayoutDetection};
use crate::llm::region_extractor::extract_region_with_vlm;
use crate::pdf::structure::types::{LayoutHint, LayoutHintClass};
use crate::types::BoundingBox;
use crate::types::document_structure::ContentLayer;
use crate::types::internal::{ElementKind, InternalDocument, InternalElement, RelationshipKind};

/// Minimum confidence threshold for a layout hint to trigger VLM region extraction.
const MIN_REGION_CONFIDENCE: f32 = 0.6;

/// Minimum pixel area for a cropped region to be sent to VLM.
///
/// Regions smaller than this are skipped — they are unlikely to contain meaningful
/// content and would waste VLM API tokens.
const MIN_REGION_PIXEL_AREA: u32 = 1_000;

/// Share of a native element's area that must lie inside a table or other region for
/// the region's answer to replace it. Above one half, an element belongs to at most one
/// of two disjoint regions, and the margin absorbs jitter between the layout box and the
/// native geometry. An element straddling the edge is kept: duplicated text over lost text.
const REPLACE_CONTAINMENT: f64 = 0.6;

/// A VLM-extracted markdown string and the page index it belongs to.
pub(crate) struct RegionVlmResult {
    /// 0-based page index.
    pub page_index: usize,
    /// Markdown text from the VLM.
    pub markdown: String,
    /// The layout hint that triggered this extraction. Used by [`inject_region_results`]
    /// to splice the result at the region's original position rather than at the end
    /// of the page.
    pub hint: LayoutHint,
}

/// Maps a layout hint class to the [`RegionKind`] that should handle it via VLM,
/// or `None` if the class should stay on the classical extraction path.
///
/// - [`LayoutHintClass::Picture`] → [`RegionKind::Figure`]: figures/diagrams never
///   have usable classical text.
/// - [`LayoutHintClass::Table`] → [`RegionKind::DenseTable`]: classical table
///   reconstruction is prone to garbling dense/borderless tables.
/// - [`LayoutHintClass::Other`] → [`RegionKind::ComplexLayout`]: by definition, a
///   region the classical layout classifier could not categorise.
///
/// Wrapper classes with dedicated classical handling (`Form`, `KeyValueRegion`,
/// `DocumentIndex`) are intentionally excluded to avoid double-processing regions
/// that already extract cleanly.
const fn region_kind_for_hint(class_name: LayoutHintClass) -> Option<RegionKind> {
    match class_name {
        LayoutHintClass::Picture => Some(RegionKind::Figure),
        LayoutHintClass::Table => Some(RegionKind::DenseTable),
        LayoutHintClass::Other => Some(RegionKind::ComplexLayout),
        _ => None,
    }
}

/// Whether a region's answer stands in for the native content inside it. A figure's
/// description does not transcribe the labels or text drawn in the figure.
const fn answer_replaces_native(class_name: LayoutHintClass) -> bool {
    matches!(class_name, LayoutHintClass::Table | LayoutHintClass::Other)
}

/// Extract markdown from VLM for all figure / complex-layout regions across all pages.
///
/// Iterates the layout hints for each page. For hints whose class warrants VLM
/// extraction ([`LayoutHintClass::Picture`] and, when requested, dense tables),
/// crops the region from the page raster at the hint's pixel-space detection in
/// `layout_detections`, encodes it as PNG, and calls
/// [`extract_region_with_vlm`]. Results from all pages are collected and returned.
///
/// # Arguments
///
/// * `layout_images` — One rasterised `RgbImage` per page (same order as `layout_hints`).
/// * `layout_hints` — Per-page layout detection results, each a `Vec<LayoutHint>`.
/// * `layout_detections` — The pixel-space detections each page's hints were converted from, one to one.
/// * `llm_config` — LLM provider configuration for the VLM calls.
///
/// # Returns
///
/// A `Vec<RegionVlmResult>` with one entry per successfully extracted region.
/// Errors from individual regions are logged as warnings but do not propagate —
/// a VLM failure on one region must not abort the whole extraction.
pub(crate) async fn extract_vlm_regions(
    layout_images: &[image::RgbImage],
    layout_hints: &[Vec<LayoutHint>],
    layout_detections: &[DetectionResult],
    llm_config: &LlmConfig,
) -> Vec<RegionVlmResult> {
    let mut results: Vec<RegionVlmResult> = Vec::new();

    for (page_index, ((page_image, hints), detection)) in layout_images
        .iter()
        .zip(layout_hints.iter())
        .zip(layout_detections.iter())
        .enumerate()
    {
        let Some(detections) = paired_detections(page_index, hints, detection) else {
            continue;
        };
        for (hint, det) in hints.iter().zip(detections) {
            if let Some(result) = extract_vlm_region(page_index, page_image, hint, det, llm_config).await {
                results.push(result);
            }
        }
    }

    results
}

/// Returns a page's layout detections only when they pair one to one with its hints.
fn paired_detections<'a>(
    page_index: usize,
    hints: &[LayoutHint],
    detection: &'a DetectionResult,
) -> Option<&'a [LayoutDetection]> {
    if hints.len() != detection.detections.len() {
        tracing::warn!(
            page = page_index,
            hints = hints.len(),
            detections = detection.detections.len(),
            "layout hints and detections disagree; skipping VLM regions on this page"
        );
        return None;
    }
    Some(&detection.detections)
}

/// PNG-encodes a cropped region, logging and returning `None` on encode failure so a
/// single bad crop suppresses only that region rather than aborting extraction. Split
/// out of [`extract_vlm_region`] purely to shorten that function.
fn encode_region_png(page_index: usize, crop: &image::RgbImage) -> Option<Vec<u8>> {
    let mut png_buf = Cursor::new(Vec::<u8>::new());
    let encode_result = image::codecs::png::PngEncoder::new(&mut png_buf).write_image(
        crop.as_raw(),
        crop.width(),
        crop.height(),
        ExtendedColorType::Rgb8,
    );
    if let Err(e) = encode_result {
        tracing::warn!(
            page = page_index,
            error = %e,
            "failed to PNG-encode region crop; skipping VLM call"
        );
        return None;
    }
    Some(png_buf.into_inner())
}

/// Returns the region kind and the pixel crop `(x, y, w, h)` for `hint`, cut from its detection `det`,
/// or `None` when the hint is below confidence or ineligible for VLM handling, or the crop is empty
/// or under `MIN_REGION_PIXEL_AREA`.
fn region_crop(
    page_index: usize,
    page_image: &image::RgbImage,
    hint: &LayoutHint,
    det: &LayoutDetection,
) -> Option<(RegionKind, (u32, u32, u32, u32))> {
    if hint.confidence < MIN_REGION_CONFIDENCE {
        return None;
    }

    let region_kind = region_kind_for_hint(hint.class_name)?;

    let (x_left, y_top, crop_w, crop_h) = det.bbox.clamp_to_image(page_image.width(), page_image.height())?;

    if crop_w * crop_h < MIN_REGION_PIXEL_AREA {
        tracing::trace!(
            page = page_index,
            crop_w,
            crop_h,
            "region too small for VLM extraction; skipping"
        );
        return None;
    }

    Some((region_kind, (x_left, y_top, crop_w, crop_h)))
}

/// Extracts a single layout hint's region via VLM.
async fn extract_vlm_region(
    page_index: usize,
    page_image: &image::RgbImage,
    hint: &LayoutHint,
    det: &LayoutDetection,
    llm_config: &LlmConfig,
) -> Option<RegionVlmResult> {
    let (region_kind, (x_left, y_top, crop_w, crop_h)) = region_crop(page_index, page_image, hint, det)?;

    let crop = image::imageops::crop_imm(page_image, x_left, y_top, crop_w, crop_h).to_image();
    let crop_bytes = encode_region_png(page_index, &crop)?;

    tracing::debug!(
        page = page_index,
        region_kind = ?region_kind,
        confidence = hint.confidence,
        crop_w,
        crop_h,
        "sending region to VLM"
    );

    match extract_region_with_vlm(&crop_bytes, "image/png", region_kind, llm_config, None).await {
        Ok(markdown) => {
            let trimmed = markdown.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(RegionVlmResult {
                    page_index,
                    markdown: trimmed,
                    hint: hint.clone(),
                })
            }
        }
        Err(e) => {
            tracing::warn!(
                page = page_index,
                region_kind = ?region_kind,
                error = %e,
                "VLM region extraction failed; region suppressed"
            );
            None
        }
    }
}

/// Finds the index in `elements` at which a region with top edge `hint_top` (PDF
/// coordinate space, y=0 at page bottom) belonging to `page_num` should be spliced.
///
/// Scans elements belonging to `page_num` and returns the index of the first one
/// whose bounding box starts below `hint_top` (i.e. the region sits above it in
/// reading order) — the new element is inserted right before that element. If no
/// element on the page has a bbox positioned below the region, the region is
/// inserted immediately after the last element of that page. If the page has no
/// elements at all, it is inserted right before the first element of the next
/// page, or at the end of the document if there is none.
fn find_insertion_index(elements: &[crate::types::internal::InternalElement], page_num: u32, hint_top: f32) -> usize {
    let mut insert_at = elements.len();
    for (index, element) in elements.iter().enumerate() {
        match element.page {
            Some(page) if page == page_num => {
                if let Some(bbox) = element.bbox
                    && (bbox.y1 as f32) < hint_top
                {
                    return index;
                }
                insert_at = index + 1;
            }
            Some(page) if page > page_num => return insert_at.min(index),
            _ => {}
        }
    }
    insert_at
}

/// Shifts every `Relationship` index at or past `from_index` by one position.
///
/// `InternalDocument::elements` is documented as append-only during extraction
/// precisely because `Relationship::source` and `RelationshipTarget::Index` store
/// raw positions into that vector (e.g. caption-to-figure links set up in
/// `pdf::structure::assembly`). Splicing a new element into the middle of
/// `elements` — as [`inject_region_results`] does to land VLM output at its
/// anchor — must shift those already-recorded indices in lockstep, or the
/// relationship silently starts pointing at the wrong element.
fn shift_relationship_indices(document: &mut crate::types::internal::InternalDocument, from_index: usize) {
    use crate::types::internal::RelationshipTarget;

    let Ok(from_index) = u32::try_from(from_index) else {
        return;
    };
    for relationship in &mut document.relationships {
        if relationship.source >= from_index {
            relationship.source += 1;
        }
        if let RelationshipTarget::Index(target_index) = &mut relationship.target
            && *target_index >= from_index
        {
            *target_index += 1;
        }
    }
}

/// Inserts `element` at `at` and returns its final index. The assembly emits page breaks
/// only between pages with content, so the first element on an empty page (a scan) gets
/// its own breaks; without them it is attributed to the previous page. Container starts
/// carry no page, so it goes before any that open the next page, not inside its list.
fn splice_element(
    document: &mut crate::types::internal::InternalDocument,
    mut at: usize,
    element: crate::types::internal::InternalElement,
) -> usize {
    let is_content = |element: &InternalElement| !matches!(element.kind, ElementKind::PageBreak);
    let first_on_page = element.page.is_some()
        && !document
            .elements
            .iter()
            .any(|existing| existing.page == element.page && is_content(existing));
    if first_on_page {
        while at > 0 && document.elements[at - 1].kind.is_container_start() && document.elements[at - 1].page.is_none()
        {
            at -= 1;
        }
    }
    if first_on_page && at > 0 && is_content(&document.elements[at - 1]) {
        insert_element(document, at, InternalElement::text(ElementKind::PageBreak, "", 0));
        at += 1;
    }
    insert_element(document, at, element);
    if first_on_page && document.elements.get(at + 1).is_some_and(is_content) {
        insert_element(document, at + 1, InternalElement::text(ElementKind::PageBreak, "", 0));
    }
    at
}

fn insert_element(
    document: &mut crate::types::internal::InternalDocument,
    at: usize,
    element: crate::types::internal::InternalElement,
) {
    shift_relationship_indices(document, at);
    document.elements.insert(at, element);
}

/// Share of `inner`'s area inside the hint's box, both in PDF points.
fn contained_share(inner: &BoundingBox, hint: &LayoutHint) -> f64 {
    let (left, right) = (
        f64::from(hint.left.min(hint.right)),
        f64::from(hint.left.max(hint.right)),
    );
    let (bottom, top) = (
        f64::from(hint.bottom.min(hint.top)),
        f64::from(hint.bottom.max(hint.top)),
    );
    let (x0, x1) = (inner.x0.min(inner.x1), inner.x0.max(inner.x1));
    let (y0, y1) = (inner.y0.min(inner.y1), inner.y0.max(inner.y1));
    let area = (x1 - x0) * (y1 - y0);
    let width = (x1.min(right) - x0.max(left)).max(0.0);
    let height = (y1.min(top) - y0.max(bottom)).max(0.0);
    if area > 0.0 { width * height / area } else { 0.0 }
}

/// Page and box of an element; a table element without its own falls back to its table's.
fn element_page_box(document: &InternalDocument, element: &InternalElement) -> Option<(u32, BoundingBox)> {
    let table = match element.kind {
        ElementKind::Table { table_index } => document.tables.get(table_index as usize),
        _ => None,
    };
    let page = element.page.or(table.map(|table| table.page_number))?;
    let bbox = element.bbox.or(table.and_then(|table| table.bounding_box))?;
    Some((page, bbox))
}

/// Whether `element` is mostly inside the region's box on its page.
fn inside_region(document: &InternalDocument, element: &InternalElement, page_num: u32, hint: &LayoutHint) -> bool {
    element_page_box(document, element)
        .is_some_and(|(page, bbox)| page == page_num && contained_share(&bbox, hint) >= REPLACE_CONTAINMENT)
}

/// Whether an answer may stand in for `element`: a table, or a body paragraph. A paragraph that is
/// linked as a caption stays; any other body paragraph inside the region, an unlinked caption
/// included, is replaced, because the region's image is what the VLM transcribed. Everything else
/// stays, since a wrong guess there would lose text the answer does not carry.
fn replaceable(element: &InternalElement, is_caption: bool) -> bool {
    match element.kind {
        ElementKind::Table { .. } => true,
        ElementKind::Paragraph => !is_caption && element.layer == ContentLayer::Body,
        _ => false,
    }
}

/// Also drops each container start and end pair the removal left with nothing inside, so the
/// answer is not spliced into an empty group and the page reads as empty to [`splice_element`].
fn drop_emptied_containers(elements: &[InternalElement], drop: &mut [bool]) {
    // (start index, holds a kept element, holds a dropped one)
    let mut open: Vec<(usize, bool, bool)> = Vec::new();
    for (index, element) in elements.iter().enumerate() {
        if element.kind.is_container_start() {
            open.push((index, false, false));
            continue;
        }
        let (kept, dropped) = if element.kind.is_container_end()
            && let Some((start, kept, dropped)) = open.pop()
        {
            let emptied = dropped && !kept;
            if emptied {
                drop[start] = true;
                drop[index] = true;
            }
            (!emptied, emptied)
        } else {
            (!drop[index], drop[index])
        };
        if let Some(parent) = open.last_mut() {
            parent.1 |= kept;
            parent.2 |= dropped;
        }
    }
}

/// Also drops a page break left leading or doubled by the removal, so a page emptied by it
/// reads as empty to [`splice_element`]. A trailing one needs no drop: the answer follows it.
fn drop_emptied_page_breaks(elements: &[InternalElement], drop: &mut [bool]) {
    let mut last_kept: Option<usize> = None;
    let mut removed_since = false;
    for (index, element) in elements.iter().enumerate() {
        if drop[index] {
            removed_since = true;
            continue;
        }
        let is_break = matches!(element.kind, ElementKind::PageBreak);
        let after_break = last_kept.is_none_or(|kept| matches!(elements[kept].kind, ElementKind::PageBreak));
        if is_break && removed_since && after_break {
            drop[index] = true;
            continue;
        }
        last_kept = Some(index);
        removed_since = false;
    }
}

/// Removes the elements flagged in `drop` and every relationship whose source or `Index` target is
/// one of them; the remaining indices are remapped. Not the OCR path's `remap_relationships`: it also drops
/// `Key` links that never resolved, which `resolve_relationships` reports in a warning later.
fn remove_elements(document: &mut InternalDocument, drop: &[bool]) {
    use crate::types::internal::RelationshipTarget;

    let mut next = 0u32;
    let new_index: Vec<Option<u32>> = drop
        .iter()
        .map(|dropped| {
            (!dropped).then(|| {
                next += 1;
                next - 1
            })
        })
        .collect();
    let mut flags = drop.iter();
    document.elements.retain(|_| !flags.next().copied().unwrap_or(false));
    document.relationships.retain_mut(|relationship| {
        let Some(source) = new_index.get(relationship.source as usize).copied().flatten() else {
            return false;
        };
        relationship.source = source;
        if let RelationshipTarget::Index(target) = &mut relationship.target {
            let Some(remapped) = new_index.get(*target as usize).copied().flatten() else {
                return false;
            };
            *target = remapped;
        }
        true
    });
}

/// Removes the native content a table or other region's answer replaces from the document elements.
/// `document.tables` and any prebuilt pages are not touched, so the `tables` list keeps the native
/// table, as do prebuilt `pages[n].tables` and Plain or Json page content; page content rendered
/// from the elements, and pages built from them, show the answer.
fn remove_region_content(document: &mut InternalDocument, page_num: u32, hint: &LayoutHint) {
    let captions: ahash::AHashSet<u32> = document
        .relationships
        .iter()
        .filter(|relationship| matches!(relationship.kind, RelationshipKind::Caption))
        .map(|relationship| relationship.source)
        .collect();
    let mut drop: Vec<bool> = document
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            replaceable(element, captions.contains(&(index as u32))) && inside_region(document, element, page_num, hint)
        })
        .collect();
    if !drop.contains(&true) {
        return;
    }
    drop_emptied_containers(&document.elements, &mut drop);
    drop_emptied_page_breaks(&document.elements, &mut drop);
    remove_elements(document, &drop);
}

/// Inject VLM-extracted region markdown into an `InternalDocument`.
///
/// For each `RegionVlmResult`, splices a paragraph element carrying the VLM markdown
/// into `document.elements` at the position of the originating layout hint, using
/// [`find_insertion_index`] to locate it among the page's existing elements by
/// bounding-box geometry, rather than appending it after the page's content.
/// A table or other region's answer first replaces the native table elements and body paragraphs
/// mostly inside it ([`REPLACE_CONTAINMENT`]); a figure's is added beside them.
/// Existing `Relationship` indices are remapped so caption/link relationships
/// recorded before the splice keep pointing at the correct element.
pub(crate) fn inject_region_results(document: &mut InternalDocument, results: Vec<RegionVlmResult>) {
    for result in results {
        let page_num = (result.page_index + 1) as u32;
        if answer_replaces_native(result.hint.class_name) {
            remove_region_content(document, page_num, &result.hint);
        }
        let insert_at = find_insertion_index(&document.elements, page_num, result.hint.top);
        let inserted_at = splice_element(
            document,
            insert_at,
            InternalElement::text(ElementKind::Paragraph, result.markdown, 0).with_page(page_num),
        );
        tracing::debug!(page = page_num, inserted_at, "injected VLM region result into document");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{BBox, LayoutClass};
    use crate::pdf::structure::types::LayoutHintClass;

    fn make_hint(class: LayoutHintClass, confidence: f32) -> LayoutHint {
        LayoutHint {
            class_name: class,
            confidence,
            left: 50.0,
            bottom: 600.0,
            right: 400.0,
            top: 750.0,
        }
    }

    #[test]
    fn test_low_confidence_hints_are_skipped() {
        let hint = make_hint(LayoutHintClass::Picture, 0.3);
        assert!(hint.confidence < MIN_REGION_CONFIDENCE);
    }

    #[test]
    fn should_skip_hints_with_dedicated_classical_handling() {
        let classically_handled = [
            LayoutHintClass::Text,
            LayoutHintClass::SectionHeader,
            LayoutHintClass::Title,
            LayoutHintClass::PageHeader,
            LayoutHintClass::PageFooter,
            LayoutHintClass::Caption,
            LayoutHintClass::Code,
            LayoutHintClass::Formula,
            LayoutHintClass::Footnote,
            LayoutHintClass::ListItem,
            LayoutHintClass::Form,
            LayoutHintClass::KeyValueRegion,
            LayoutHintClass::DocumentIndex,
        ];
        for class in classically_handled {
            assert_eq!(
                region_kind_for_hint(class),
                None,
                "{class:?} must not be routed to the VLM"
            );
        }
    }

    #[test]
    fn should_route_dense_table_hint_to_dense_table_region_kind() {
        assert_eq!(
            region_kind_for_hint(LayoutHintClass::Table),
            Some(RegionKind::DenseTable)
        );
    }

    #[test]
    fn should_route_other_hint_to_complex_layout_region_kind() {
        assert_eq!(
            region_kind_for_hint(LayoutHintClass::Other),
            Some(RegionKind::ComplexLayout)
        );
    }

    #[test]
    fn should_route_picture_hint_to_figure_region_kind() {
        assert_eq!(region_kind_for_hint(LayoutHintClass::Picture), Some(RegionKind::Figure));
    }

    #[test]
    fn should_crop_the_pixel_detection_not_the_pdf_space_hint() {
        // US Letter (612x792 pt) rendered at 150 dpi.
        let page_image = image::RgbImage::new(1_275, 1_650);
        let det = LayoutDetection::new(LayoutClass::Table, 0.9, BBox::new(100.0, 200.0, 700.0, 500.0));
        let hint = LayoutHint {
            class_name: LayoutHintClass::Table,
            confidence: 0.9,
            left: 48.0,
            bottom: 552.0,
            right: 336.0,
            top: 696.0,
        };

        let (kind, rect) = region_crop(0, &page_image, &hint, &det).expect("confident table region should be cropped");
        assert_eq!(kind, RegionKind::DenseTable);
        assert_eq!(rect, (100, 200, 600, 300));
    }

    #[test]
    fn should_skip_detection_boxes_that_clamp_to_nothing() {
        let page_image = image::RgbImage::new(1_275, 1_650);
        let hint = make_hint(LayoutHintClass::Table, 0.9);
        for bbox in [
            BBox::new(100.0, 200.0, 100.0, 500.0),
            BBox::new(-500.0, -500.0, -100.0, -100.0),
        ] {
            let det = LayoutDetection::new(LayoutClass::Table, 0.9, bbox);
            assert!(region_crop(0, &page_image, &hint, &det).is_none());
        }
    }

    #[test]
    fn should_skip_page_when_hint_and_detection_counts_disagree() {
        let hints = [make_hint(LayoutHintClass::Table, 0.9)];
        let det = LayoutDetection::new(LayoutClass::Table, 0.9, BBox::new(100.0, 200.0, 700.0, 500.0));

        assert!(paired_detections(0, &hints, &DetectionResult::new(1_275, 1_650, vec![])).is_none());
        assert!(
            paired_detections(
                0,
                &hints,
                &DetectionResult::new(1_275, 1_650, vec![det.clone(), det.clone()])
            )
            .is_none()
        );
        assert_eq!(
            paired_detections(0, &hints, &DetectionResult::new(1_275, 1_650, vec![det])).map(<[_]>::len),
            Some(1)
        );
    }

    #[test]
    fn test_min_pixel_area_constant() {
        const { assert!(MIN_REGION_PIXEL_AREA > 0) };
    }

    fn element_with_bbox(page: u32, top: f64) -> crate::types::internal::InternalElement {
        use crate::types::BoundingBox;
        use crate::types::internal::{ElementKind, InternalElement};

        let mut element = InternalElement::text(ElementKind::Paragraph, "existing", 0);
        element.page = Some(page);
        element.bbox = Some(BoundingBox {
            x0: 0.0,
            y0: top - 10.0,
            x1: 100.0,
            y1: top,
        });
        element
    }

    fn hint_with_top(top: f32) -> LayoutHint {
        LayoutHint {
            class_name: LayoutHintClass::Table,
            confidence: 0.9,
            left: 50.0,
            bottom: top - 100.0,
            right: 400.0,
            top,
        }
    }

    #[test]
    fn should_splice_vlm_result_at_its_bbox_anchor_not_at_document_end() {
        use crate::types::internal::InternalDocument;

        let mut document = InternalDocument::default();
        // Page 1: a paragraph above the region (top=750) and one below it (top=400).
        document.elements.push(element_with_bbox(1, 750.0));
        document.elements.push(element_with_bbox(1, 400.0));
        // Page 2: an unrelated paragraph, which must stay after the injected result.
        document.elements.push(element_with_bbox(2, 700.0));

        let results = vec![RegionVlmResult {
            page_index: 0,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(document.elements.len(), 4);
        assert_eq!(document.elements[0].text, "existing");
        assert_eq!(document.elements[0].page, Some(1));
        assert_eq!(document.elements[1].text, "VLM TABLE");
        assert_eq!(document.elements[1].page, Some(1));
        assert_eq!(document.elements[2].text, "existing");
        assert_eq!(document.elements[2].page, Some(1));
        assert_eq!(document.elements[3].page, Some(2));
    }

    #[test]
    fn should_append_vlm_result_after_page_when_no_element_bbox_is_below_it() {
        use crate::types::internal::InternalDocument;

        let mut document = InternalDocument::default();
        document.elements.push(element_with_bbox(1, 750.0));
        document.elements.push(element_with_bbox(1, 700.0));

        let results = vec![RegionVlmResult {
            page_index: 0,
            markdown: "VLM CAPTION".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(document.elements.len(), 3);
        assert_eq!(document.elements[2].text, "VLM CAPTION");
    }

    fn page_break() -> crate::types::internal::InternalElement {
        use crate::types::internal::{ElementKind, InternalElement};

        InternalElement::text(ElementKind::PageBreak, "", 0)
    }

    fn page_shape(document: &crate::types::internal::InternalDocument) -> Vec<(Option<u32>, bool)> {
        use crate::types::internal::ElementKind;

        document
            .elements
            .iter()
            .map(|element| (element.page, matches!(element.kind, ElementKind::PageBreak)))
            .collect()
    }

    #[test]
    fn should_break_before_vlm_result_on_trailing_page_with_no_elements() {
        use crate::types::internal::InternalDocument;

        let mut document = InternalDocument::default();
        document.elements.push(element_with_bbox(1, 700.0));

        let results = vec![RegionVlmResult {
            page_index: 1,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(
            page_shape(&document),
            vec![(Some(1), false), (None, true), (Some(2), false)]
        );
        assert_eq!(document.elements[2].text, "VLM TABLE");
    }

    #[test]
    fn should_give_vlm_result_on_page_with_no_elements_breaks_on_both_sides() {
        use crate::types::internal::{InternalDocument, Relationship, RelationshipKind, RelationshipTarget};

        let mut document = InternalDocument::default();
        document.elements.push(element_with_bbox(1, 700.0));
        document.elements.push(page_break());
        document.elements.push(element_with_bbox(3, 700.0));
        document.relationships.push(Relationship {
            source: 0,
            target: RelationshipTarget::Index(2),
            kind: RelationshipKind::InternalLink,
        });

        let results = vec![
            RegionVlmResult {
                page_index: 1,
                markdown: "VLM FIGURE".to_string(),
                hint: hint_with_top(600.0),
            },
            RegionVlmResult {
                page_index: 1,
                markdown: "VLM TABLE".to_string(),
                hint: hint_with_top(300.0),
            },
        ];

        inject_region_results(&mut document, results);

        assert_eq!(
            page_shape(&document),
            vec![
                (Some(1), false),
                (None, true),
                (Some(2), false),
                (Some(2), false),
                (None, true),
                (Some(3), false)
            ]
        );
        assert_eq!(document.elements[2].text, "VLM FIGURE");
        assert_eq!(document.elements[3].text, "VLM TABLE");
        assert_eq!(
            document.relationships[0].target,
            RelationshipTarget::Index(5),
            "the page-3 target must follow both results and the added break"
        );
    }

    fn list_marker(kind: crate::types::internal::ElementKind) -> crate::types::internal::InternalElement {
        crate::types::internal::InternalElement::text(kind, "", 0)
    }

    fn list_item(page: u32) -> crate::types::internal::InternalElement {
        use crate::types::internal::{ElementKind, InternalElement};

        InternalElement::text(ElementKind::ListItem { ordered: false }, "item", 0).with_page(page)
    }

    #[test]
    fn should_place_vlm_result_on_leading_empty_page_before_next_page_list() {
        use crate::types::internal::{
            ElementKind, InternalDocument, Relationship, RelationshipKind, RelationshipTarget,
        };

        let mut document = InternalDocument::default();
        document
            .elements
            .push(list_marker(ElementKind::ListStart { ordered: false }));
        document.elements.push(list_item(2));
        document.elements.push(list_marker(ElementKind::ListEnd));
        document.relationships.push(Relationship {
            source: 1,
            target: RelationshipTarget::Index(1),
            kind: RelationshipKind::InternalLink,
        });

        let results = vec![RegionVlmResult {
            page_index: 0,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(
            page_shape(&document),
            vec![
                (Some(1), false),
                (None, true),
                (None, false),
                (Some(2), false),
                (None, false)
            ]
        );
        assert_eq!(document.elements[0].text, "VLM TABLE");
        assert!(matches!(document.elements[2].kind, ElementKind::ListStart { .. }));
        assert_eq!(document.relationships[0].source, 3);
        assert_eq!(document.relationships[0].target, RelationshipTarget::Index(3));
    }

    #[test]
    fn should_place_vlm_result_on_empty_page_before_next_page_list() {
        use crate::types::internal::{ElementKind, InternalDocument};

        let mut document = InternalDocument::default();
        document.elements.push(element_with_bbox(1, 700.0));
        document.elements.push(page_break());
        document
            .elements
            .push(list_marker(ElementKind::ListStart { ordered: false }));
        document.elements.push(list_item(3));

        let results = vec![RegionVlmResult {
            page_index: 1,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(
            page_shape(&document),
            vec![
                (Some(1), false),
                (None, true),
                (Some(2), false),
                (None, true),
                (None, false),
                (Some(3), false)
            ]
        );
        assert_eq!(document.elements[2].text, "VLM TABLE");
        assert!(matches!(document.elements[4].kind, ElementKind::ListStart { .. }));
    }

    #[test]
    fn should_break_before_vlm_result_after_last_page_closing_list() {
        use crate::types::internal::{ElementKind, InternalDocument};

        let mut document = InternalDocument::default();
        document
            .elements
            .push(list_marker(ElementKind::ListStart { ordered: false }));
        document.elements.push(list_item(1));
        document.elements.push(list_marker(ElementKind::ListEnd));

        let results = vec![RegionVlmResult {
            page_index: 1,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        assert_eq!(
            page_shape(&document),
            vec![
                (None, false),
                (Some(1), false),
                (None, false),
                (None, true),
                (Some(2), false)
            ]
        );
        assert_eq!(document.elements[4].text, "VLM TABLE");
    }

    #[test]
    fn should_shift_relationship_indices_when_splicing_before_referenced_elements() {
        use crate::types::internal::{InternalDocument, Relationship, RelationshipKind, RelationshipTarget};

        let mut document = InternalDocument::default();
        // index 0: paragraph above the region (stays before the splice point)
        document.elements.push(element_with_bbox(1, 750.0));
        // index 1: paragraph below the region (splice must land before this one)
        document.elements.push(element_with_bbox(1, 400.0));
        // index 2: unrelated element on page 2
        document.elements.push(element_with_bbox(2, 700.0));

        // A caption relationship recorded before splicing, pointing at raw indices.
        document.relationships.push(Relationship {
            source: 1,
            target: RelationshipTarget::Index(2),
            kind: RelationshipKind::Caption,
        });
        // A relationship entirely before the splice point must be left untouched.
        document.relationships.push(Relationship {
            source: 0,
            target: RelationshipTarget::Key("unresolved".to_string()),
            kind: RelationshipKind::InternalLink,
        });

        let results = vec![RegionVlmResult {
            page_index: 0,
            markdown: "VLM TABLE".to_string(),
            hint: hint_with_top(600.0),
        }];

        inject_region_results(&mut document, results);

        // The new element was inserted at index 1, so indices >= 1 shift by one.
        assert_eq!(document.elements[1].text, "VLM TABLE");
        assert_eq!(document.elements[2].text, "existing");
        assert_eq!(document.relationships[0].source, 2, "source index 1 must shift to 2");
        assert_eq!(
            document.relationships[0].target,
            RelationshipTarget::Index(3),
            "target index 2 must shift to 3"
        );
        assert_eq!(
            document.relationships[1].source, 0,
            "relationship before the splice point must not shift"
        );
        assert_eq!(
            document.relationships[1].target,
            RelationshipTarget::Key("unresolved".to_string()),
            "unresolved key targets are untouched"
        );
    }

    fn region_hint(class_name: LayoutHintClass, bottom: f32, top: f32) -> LayoutHint {
        LayoutHint {
            class_name,
            confidence: 0.9,
            left: 50.0,
            bottom,
            right: 550.0,
            top,
        }
    }

    fn page_box(bottom: f64, top: f64) -> BoundingBox {
        BoundingBox {
            x0: 60.0,
            y0: bottom,
            x1: 540.0,
            y1: top,
        }
    }

    fn paragraph_at(page: u32, text: &str, bottom: f64, top: f64) -> InternalElement {
        let mut element = InternalElement::text(ElementKind::Paragraph, text, 0).with_page(page);
        element.bbox = Some(page_box(bottom, top));
        element
    }

    fn table_at(document: &mut InternalDocument, page: u32, id: &str, bottom: f64, top: f64) -> InternalElement {
        let table_index = document.push_table(crate::types::Table {
            cells: vec![vec![id.to_string()]],
            markdown: format!("| {id} |"),
            page_number: page,
            bounding_box: Some(page_box(bottom, top)),
            ..Default::default()
        });
        let mut element = InternalElement::text(ElementKind::Table { table_index }, "", 0).with_page(page);
        element.bbox = Some(page_box(bottom, top));
        element
    }

    fn region_result(page_index: usize, markdown: &str, hint: LayoutHint) -> RegionVlmResult {
        RegionVlmResult {
            page_index,
            markdown: markdown.to_string(),
            hint,
        }
    }

    fn texts(document: &InternalDocument) -> Vec<String> {
        document
            .elements
            .iter()
            .map(|element| match element.kind {
                ElementKind::Table { table_index } => {
                    format!("table:{}", document.tables[table_index as usize].markdown)
                }
                ElementKind::PageBreak => "break".to_string(),
                _ => element.text.clone(),
            })
            .collect()
    }

    /// Page 1: a caption above a table region, the native table and a stray row read as text
    /// inside it, a paragraph below. Page 2: a table at the same coordinates.
    fn table_region_document() -> InternalDocument {
        use crate::types::internal::{Relationship, RelationshipKind, RelationshipTarget};

        let mut document = InternalDocument::default();
        let first = table_at(&mut document, 1, "native-1", 340.0, 580.0);
        let second = table_at(&mut document, 2, "native-2", 340.0, 580.0);
        document.elements.push(paragraph_at(1, "caption", 620.0, 640.0));
        document.elements.push(first);
        document
            .elements
            .push(paragraph_at(1, "row read as text", 310.0, 330.0));
        document.elements.push(paragraph_at(1, "below", 100.0, 200.0));
        document.elements.push(page_break());
        document.elements.push(second);
        document.relationships.push(Relationship {
            source: 0,
            target: RelationshipTarget::Index(1),
            kind: RelationshipKind::Caption,
        });
        document.relationships.push(Relationship {
            source: 3,
            target: RelationshipTarget::Index(5),
            kind: RelationshipKind::InternalLink,
        });
        document
    }

    #[test]
    fn should_replace_native_content_inside_table_and_other_regions_with_the_answer() {
        use crate::types::internal::RelationshipTarget;

        for class_name in [LayoutHintClass::Table, LayoutHintClass::Other] {
            let mut document = table_region_document();

            inject_region_results(
                &mut document,
                vec![region_result(0, "VLM TABLE", region_hint(class_name, 300.0, 600.0))],
            );

            assert_eq!(
                texts(&document),
                ["caption", "VLM TABLE", "below", "break", "table:| native-2 |"],
                "{class_name:?}"
            );
            assert_eq!(document.tables.len(), 2, "only the elements change: {class_name:?}");
            assert_eq!(
                document.relationships.len(),
                1,
                "the caption's link to the removed table must go: {:?}",
                document.relationships
            );
            assert_eq!(document.relationships[0].source, 2);
            assert_eq!(document.relationships[0].target, RelationshipTarget::Index(4));
        }
    }

    #[test]
    fn should_keep_native_content_beside_a_picture_answer() {
        let mut document = table_region_document();

        inject_region_results(
            &mut document,
            vec![region_result(
                0,
                "VLM FIGURE",
                region_hint(LayoutHintClass::Picture, 300.0, 600.0),
            )],
        );

        assert_eq!(
            texts(&document),
            [
                "caption",
                "VLM FIGURE",
                "table:| native-1 |",
                "row read as text",
                "below",
                "break",
                "table:| native-2 |"
            ]
        );
        assert_eq!(document.tables.len(), 2);
        assert_eq!(document.relationships.len(), 2);
    }

    #[test]
    fn should_keep_elements_and_tables_mostly_outside_the_region() {
        let mut document = InternalDocument::default();
        let straddling_table = table_at(&mut document, 1, "straddling", 100.0, 580.0);
        document.elements.push(straddling_table);
        document.elements.push(paragraph_at(1, "half inside", 250.0, 350.0));
        document.elements.push(paragraph_at(1, "mostly inside", 270.0, 370.0));

        inject_region_results(
            &mut document,
            vec![region_result(
                0,
                "VLM TABLE",
                region_hint(LayoutHintClass::Table, 300.0, 600.0),
            )],
        );

        assert_eq!(
            texts(&document),
            ["VLM TABLE", "table:| straddling |", "half inside"],
            "a table 58% inside and a paragraph 50% inside stay; one 70% inside goes"
        );
    }

    #[test]
    fn should_keep_a_table_split_evenly_between_two_regions() {
        let mut document = InternalDocument::default();
        let table = table_at(&mut document, 1, "split", 340.0, 580.0);
        document.elements.push(table);

        inject_region_results(
            &mut document,
            vec![
                region_result(0, "UPPER", region_hint(LayoutHintClass::Table, 460.0, 600.0)),
                region_result(0, "LOWER", region_hint(LayoutHintClass::Table, 300.0, 460.0)),
            ],
        );
        assert_eq!(
            texts(&document),
            ["UPPER", "table:| split |", "LOWER"],
            "a table split evenly between two regions belongs to neither"
        );
    }

    #[test]
    fn should_find_a_flat_table_element_by_its_table_page_and_box() {
        let mut document = InternalDocument::default();
        let table_index = document.push_table(crate::types::Table {
            markdown: "| flat |".to_string(),
            page_number: 1,
            bounding_box: Some(page_box(340.0, 580.0)),
            ..Default::default()
        });
        document
            .elements
            .push(InternalElement::text(ElementKind::Table { table_index }, "", 0));

        inject_region_results(
            &mut document,
            vec![region_result(
                0,
                "VLM TABLE",
                region_hint(LayoutHintClass::Table, 300.0, 600.0),
            )],
        );

        assert_eq!(texts(&document), ["VLM TABLE"]);
    }

    #[test]
    fn should_keep_images_inside_an_other_region() {
        let mut document = InternalDocument::default();
        let mut image = InternalElement::text(ElementKind::Image { image_index: 0 }, "", 0).with_page(1);
        image.bbox = Some(page_box(340.0, 580.0));
        document.elements.push(image);

        inject_region_results(
            &mut document,
            vec![region_result(
                0,
                "VLM LAYOUT",
                region_hint(LayoutHintClass::Other, 300.0, 600.0),
            )],
        );

        assert_eq!(document.elements.len(), 2);
        assert!(matches!(document.elements[1].kind, ElementKind::Image { .. }));
    }

    fn element_at(kind: ElementKind, page: u32, text: &str, bottom: f64, top: f64) -> InternalElement {
        let mut element = InternalElement::text(kind, text, 0).with_page(page);
        element.bbox = Some(page_box(bottom, top));
        element
    }

    #[test]
    fn should_replace_only_tables_and_body_paragraphs_inside_the_region() {
        use crate::types::internal::{Relationship, RelationshipTarget};

        let mut document = InternalDocument::default();
        let table = table_at(&mut document, 1, "native", 460.0, 580.0);
        document.elements.push(table);
        document
            .elements
            .push(element_at(ElementKind::Paragraph, 1, "cell text", 430.0, 450.0));
        document.elements.push(element_at(
            ElementKind::Heading { level: 2 },
            1,
            "heading",
            400.0,
            420.0,
        ));
        document
            .elements
            .push(element_at(ElementKind::Paragraph, 1, "caption", 380.0, 395.0));
        document
            .elements
            .push(element_at(ElementKind::Formula, 1, "E = mc^2", 360.0, 375.0));
        document
            .elements
            .push(element_at(ElementKind::Paragraph, 1, "footer", 340.0, 355.0).with_layer(ContentLayer::Footer));
        document.elements.push(element_at(
            ElementKind::ListItem { ordered: false },
            1,
            "item",
            320.0,
            335.0,
        ));
        document.relationships.push(Relationship {
            source: 3,
            target: RelationshipTarget::Index(0),
            kind: RelationshipKind::Caption,
        });

        inject_region_results(
            &mut document,
            vec![region_result(
                0,
                "VLM TABLE",
                region_hint(LayoutHintClass::Table, 300.0, 600.0),
            )],
        );

        assert_eq!(
            texts(&document),
            ["VLM TABLE", "heading", "caption", "E = mc^2", "footer", "item"]
        );
    }

    #[test]
    fn should_drop_groups_the_answer_empties_and_break_around_it() {
        let hint = || region_hint(LayoutHintClass::Table, 300.0, 600.0);
        let group_start = || InternalElement::text(ElementKind::GroupStart, "", 0).with_page(2);
        let group_end = || InternalElement::text(ElementKind::GroupEnd, "", 0);

        let mut emptied = InternalDocument::default();
        emptied.elements.push(paragraph_at(1, "first", 100.0, 200.0));
        emptied.elements.push(page_break());
        emptied.elements.push(group_start());
        emptied.elements.push(group_start());
        emptied.elements.push(paragraph_at(2, "cell text", 340.0, 580.0));
        emptied.elements.push(group_end());
        emptied.elements.push(group_end());
        emptied.elements.push(page_break());
        emptied.elements.push(paragraph_at(3, "third", 100.0, 200.0));
        inject_region_results(&mut emptied, vec![region_result(1, "VLM", hint())]);
        assert_eq!(texts(&emptied), ["first", "break", "VLM", "break", "third"]);

        let mut kept = InternalDocument::default();
        kept.elements.push(group_start());
        kept.elements.push(element_at(
            ElementKind::Heading { level: 2 },
            2,
            "heading",
            560.0,
            580.0,
        ));
        kept.elements.push(paragraph_at(2, "cell text", 340.0, 540.0));
        kept.elements.push(group_end());
        inject_region_results(&mut kept, vec![region_result(1, "VLM", hint())]);
        let kinds: Vec<_> = kept.elements.iter().map(|element| element.kind).collect();
        assert_eq!(
            kinds,
            [
                ElementKind::GroupStart,
                ElementKind::Paragraph,
                ElementKind::Heading { level: 2 },
                ElementKind::GroupEnd
            ]
        );
        assert_eq!(kept.elements[1].text, "VLM");
    }

    #[test]
    fn should_keep_one_break_on_each_side_of_a_page_the_answer_empties() {
        let hint = || region_hint(LayoutHintClass::Table, 300.0, 600.0);

        let mut middle = InternalDocument::default();
        let table = table_at(&mut middle, 2, "middle", 340.0, 580.0);
        middle.elements.push(paragraph_at(1, "first", 100.0, 200.0));
        middle.elements.push(page_break());
        middle.elements.push(table);
        middle.elements.push(page_break());
        middle.elements.push(paragraph_at(3, "third", 100.0, 200.0));
        inject_region_results(&mut middle, vec![region_result(1, "VLM", hint())]);
        assert_eq!(texts(&middle), ["first", "break", "VLM", "break", "third"]);
        assert_eq!(middle.elements[2].page, Some(2));

        let mut leading = InternalDocument::default();
        let table = table_at(&mut leading, 1, "leading", 340.0, 580.0);
        leading.elements.push(table);
        leading.elements.push(page_break());
        leading.elements.push(paragraph_at(2, "second", 100.0, 200.0));
        inject_region_results(&mut leading, vec![region_result(0, "VLM", hint())]);
        assert_eq!(texts(&leading), ["VLM", "break", "second"]);

        let mut trailing = InternalDocument::default();
        let table = table_at(&mut trailing, 2, "trailing", 340.0, 580.0);
        trailing.elements.push(paragraph_at(1, "first", 100.0, 200.0));
        trailing.elements.push(page_break());
        trailing.elements.push(table);
        inject_region_results(&mut trailing, vec![region_result(1, "VLM", hint())]);
        assert_eq!(texts(&trailing), ["first", "break", "VLM"]);
    }

    fn table_region_llm_config(base_url: String) -> LlmConfig {
        LlmConfig {
            model: "openai/gpt-4o-mini".to_string(),
            api_key: Some("test-key".to_string()),
            base_url: Some(base_url),
            timeout_secs: Some(10),
            max_retries: Some(0),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn should_leave_native_tables_alone_when_the_vlm_answer_is_empty_or_fails() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let empty_answer = serde_json::json!({
            "id": "chatcmpl-test",
            "object": "chat.completion",
            "created": 0,
            "model": "openai/gpt-4o-mini",
            "choices": [{
                "index": 0,
                "message": { "role": "assistant", "content": "  \n " },
                "finish_reason": "stop"
            }],
            "usage": { "prompt_tokens": 10, "completion_tokens": 10, "total_tokens": 20 }
        });
        for response in [
            ResponseTemplate::new(200).set_body_json(empty_answer),
            ResponseTemplate::new(500),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("POST")).respond_with(response).mount(&server).await;
            let page_image = image::RgbImage::new(1_275, 1_650);
            let hints = vec![region_hint(LayoutHintClass::Table, 300.0, 600.0)];
            let detection = DetectionResult::new(
                1_275,
                1_650,
                vec![LayoutDetection::new(
                    LayoutClass::Table,
                    0.9,
                    BBox::new(100.0, 400.0, 1_100.0, 1_000.0),
                )],
            );

            let results = extract_vlm_regions(
                &[page_image],
                &[hints],
                &[detection],
                &table_region_llm_config(server.uri()),
            )
            .await;
            let mut document = table_region_document();
            let before = texts(&document);
            inject_region_results(&mut document, results);

            assert_eq!(
                server.received_requests().await.map(|requests| requests.len()),
                Some(1),
                "the table region must reach the VLM"
            );
            assert_eq!(texts(&document), before);
            assert_eq!(document.tables.len(), 2);
            assert_eq!(document.relationships.len(), 2);
        }
    }
}
