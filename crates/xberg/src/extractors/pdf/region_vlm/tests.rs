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
        ["table:| straddling |", "half inside", "VLM TABLE"],
        "a table 58% inside and a paragraph 50% inside stay; one 70% inside goes"
    );
}

#[test]
fn should_replace_a_paragraph_whose_box_has_inverted_corners() {
    let mut inverted = paragraph_at(1, "inverted", 400.0, 500.0);
    inverted.bbox = Some(BoundingBox {
        x0: 540.0,
        y0: 500.0,
        x1: 60.0,
        y1: 400.0,
    });
    let mut document = InternalDocument::default();
    document.elements.push(inverted);
    document.elements.push(paragraph_at(1, "below", 100.0, 200.0));

    inject_region_results(
        &mut document,
        vec![region_result(
            0,
            "VLM TABLE",
            region_hint(LayoutHintClass::Table, 300.0, 600.0),
        )],
    );

    assert_eq!(texts(&document), ["VLM TABLE", "below"]);
}

#[test]
fn should_nest_a_replacing_answer_at_the_depth_of_what_it_replaced() {
    let mut nested = paragraph_at(1, "nested", 400.0, 500.0);
    nested.depth = 2;
    let mut document = InternalDocument::default();
    document.elements.push(nested);

    inject_region_results(
        &mut document,
        vec![
            region_result(0, "VLM TABLE", region_hint(LayoutHintClass::Table, 300.0, 600.0)),
            region_result(0, "VLM FIGURE", region_hint(LayoutHintClass::Picture, 300.0, 600.0)),
        ],
    );

    let depths: Vec<(&str, u16)> = document
        .elements
        .iter()
        .map(|element| (element.text.as_str(), element.depth))
        .collect();
    assert_eq!(depths, [("VLM TABLE", 2), ("VLM FIGURE", 0)]);
}

/// A table directly followed by `container`, whose start has no box: the answer must take the
/// table's place before the container, not land after its start.
fn table_followed_by(container: ElementKind, child: InternalElement, end: ElementKind) -> InternalDocument {
    let mut document = InternalDocument::default();
    let table = table_at(&mut document, 1, "native", 340.0, 580.0);
    document.elements.push(paragraph_at(1, "above", 650.0, 700.0));
    document.elements.push(table);
    document
        .elements
        .push(InternalElement::text(container, "", 0).with_page(1));
    document.elements.push(child);
    document.elements.push(InternalElement::text(end, "", 0));
    document.elements.push(paragraph_at(1, "below", 100.0, 150.0));
    document
}

fn kinds_and_texts(document: &InternalDocument) -> Vec<String> {
    document
        .elements
        .iter()
        .map(|element| format!("{}:{}", element.kind.discriminant(), element.text))
        .collect()
}

#[test]
fn should_put_a_replacing_answer_before_a_list_that_follows_the_table() {
    let mut document = table_followed_by(
        ElementKind::ListStart { ordered: false },
        element_at(ElementKind::ListItem { ordered: false }, 1, "one", 200.0, 220.0),
        ElementKind::ListEnd,
    );
    document.elements[2].page = None;

    inject_region_results(
        &mut document,
        vec![region_result(
            0,
            "VLM TABLE",
            region_hint(LayoutHintClass::Table, 300.0, 600.0),
        )],
    );

    assert_eq!(
        kinds_and_texts(&document),
        [
            "paragraph:above",
            "paragraph:VLM TABLE",
            "list_start:",
            "list_item:one",
            "list_end:",
            "paragraph:below"
        ]
    );
    assert_eq!(
        crate::rendering::render_markdown(&document),
        "above\n\nVLM TABLE\n\n- one\n\nbelow\n"
    );
}

#[test]
fn should_put_a_replacing_answer_before_a_group_that_follows_the_table() {
    let mut document = table_followed_by(
        ElementKind::GroupStart,
        paragraph_at(1, "grouped", 200.0, 220.0),
        ElementKind::GroupEnd,
    );

    inject_region_results(
        &mut document,
        vec![region_result(
            0,
            "VLM TABLE",
            region_hint(LayoutHintClass::Table, 300.0, 600.0),
        )],
    );

    assert_eq!(
        kinds_and_texts(&document),
        [
            "paragraph:above",
            "paragraph:VLM TABLE",
            "group_start:",
            "paragraph:grouped",
            "group_end:",
            "paragraph:below"
        ]
    );
}

#[test]
fn should_count_no_share_for_a_box_without_area() {
    let flat = BoundingBox {
        x0: 100.0,
        y0: 400.0,
        x1: 200.0,
        y1: 400.0,
    };

    assert_eq!(
        contained_share(&flat, &region_hint(LayoutHintClass::Table, 300.0, 600.0)),
        0.0
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
            ElementKind::Heading { level: 2 },
            ElementKind::Paragraph,
            ElementKind::GroupEnd
        ]
    );
    assert_eq!(kept.elements[2].text, "VLM");
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
