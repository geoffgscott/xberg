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

pub(super) fn page_break() -> crate::types::internal::InternalElement {
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
    use crate::types::internal::{ElementKind, InternalDocument, Relationship, RelationshipKind, RelationshipTarget};

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
