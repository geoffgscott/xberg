//! Integration tests: EPUB embedded MathML is converted to LaTeX.
//!
//! Historically `content::SKIP_ELEMENTS` dropped the entire `<math>` subtree,
//! and even where it wasn't skipped (the structural HTML walker), unrecognized
//! MathML tags fell through to plain-text accumulation, mangling formulas into
//! raw concatenated symbol soup. Both paths now route `<math>` through the
//! shared `xberg::extraction::mathml` converter and surface it as a distinct
//! `Formula` element.

#![cfg(feature = "office")]

use std::io::{Cursor, Write};
use xberg::ExtractInput;
use xberg::core::config::{ExtractionConfig, OutputFormat};
use xberg::extractors::EpubExtractor;
use xberg::plugins::DocumentExtractor;
use zip::write::FileOptions;

fn build_epub_bytes(chapter_xhtml: &str) -> Vec<u8> {
    let container_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#;

    let opf_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<package version="3.0" unique-identifier="bookid" xmlns="http://www.idpf.org/2007/opf">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>MathML Test Book</dc:title>
    <dc:language>en</dc:language>
  </metadata>
  <manifest>
    <item id="c1" href="chapter1.xhtml" media-type="application/xhtml+xml" properties="mathml"/>
  </manifest>
  <spine>
    <itemref idref="c1"/>
  </spine>
</package>"#;

    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut writer = zip::ZipWriter::new(&mut cursor);
    let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Stored);

    writer.start_file("mimetype", options).expect("zip start_file failed");
    writer
        .write_all(b"application/epub+zip")
        .expect("zip write mimetype failed");
    writer
        .add_directory("META-INF/", options)
        .expect("zip add_directory failed");
    writer
        .add_directory("OEBPS/", options)
        .expect("zip add_directory failed");

    for (path, contents) in [
        ("META-INF/container.xml", container_xml),
        ("OEBPS/content.opf", opf_xml),
        ("OEBPS/chapter1.xhtml", chapter_xhtml),
    ] {
        writer.start_file(path, options).expect("zip start_file failed");
        writer.write_all(contents.as_bytes()).expect("zip write file failed");
    }

    writer.finish().expect("zip finish failed");
    cursor.into_inner()
}

/// A chapter containing *only* a `<math>` element (no headings or paragraphs)
/// forces the structural HTML walker to produce an empty node list, which
/// exercises the plain-text fallback path in `extractors::epub::content`.
#[tokio::test]
async fn test_math_only_chapter_produces_real_latex_formula() {
    let chapter_xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <math xmlns="http://www.w3.org/1998/Math/MathML">
      <msup><mi>x</mi><mn>2</mn></msup>
    </math>
  </body>
</html>"#;
    let bytes = build_epub_bytes(chapter_xhtml);
    let extractor = EpubExtractor;
    let config = ExtractionConfig::default();
    let input = ExtractInput::from_bytes(bytes, "application/epub+zip", None);

    let result = extractor
        .extract(input, &config)
        .await
        .expect("Should extract math-only chapter successfully");

    assert!(
        result.content.contains("x^{2}"),
        "Expected the formula to render as LaTeX x^{{2}}, got: {}",
        result.content
    );
    assert!(
        !result.content.contains("msup") && !result.content.contains("mi>") && !result.content.contains("mn>"),
        "Raw MathML tag names must not leak into extracted content, got: {}",
        result.content
    );
}

/// A chapter mixing a heading, paragraph prose, and a `<math>` formula takes
/// the structural HTML walker path (`extraction::html::structure`); the
/// formula must still convert to LaTeX and the surrounding prose must survive
/// untouched.
#[tokio::test]
async fn test_mixed_chapter_converts_math_alongside_prose() {
    let chapter_xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <h1>Energy</h1>
    <p>Mass-energy equivalence:</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML">
      <mrow><mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></mrow>
    </math>
    <p>Discovered by Einstein.</p>
  </body>
</html>"#;
    let bytes = build_epub_bytes(chapter_xhtml);
    let extractor = EpubExtractor;
    let config = ExtractionConfig::default();
    let input = ExtractInput::from_bytes(bytes, "application/epub+zip", None);

    let result = extractor
        .extract(input, &config)
        .await
        .expect("Should extract mixed chapter successfully");

    assert!(
        result.content.contains("E=mc^{2}"),
        "Expected the formula to render as LaTeX E=mc^{{2}}, got: {}",
        result.content
    );
    assert!(
        result.content.contains("Mass-energy equivalence"),
        "got: {}",
        result.content
    );
    assert!(
        result.content.contains("Discovered by Einstein"),
        "got: {}",
        result.content
    );
    assert!(
        !result.content.contains("<mi>") && !result.content.contains("<mo>"),
        "Raw MathML tag names must not leak into extracted content, got: {}",
        result.content
    );
}

#[tokio::test]
async fn markdown_writes_mathml_as_delimited_latex() {
    let chapter_xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <p>Square it:</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><msup><mi>x</mi><mn>2</mn></msup></math>
    <p>so <math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mn>1</mn><mn>2</mn></mfrac></math> is half.</p>
  </body>
</html>"#;
    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        ..Default::default()
    };
    let input = ExtractInput::from_bytes(build_epub_bytes(chapter_xhtml), "application/epub+zip", None);
    let result = EpubExtractor.extract(input, &config).await.expect("extract chapter");
    let markdown = result.formatted_content.as_deref().unwrap_or(&result.content);

    assert!(markdown.contains("$$x^{2}$$"), "got: {markdown}");
    assert!(markdown.contains(r"so $\frac{1}{2}$ is half."), "got: {markdown}");
}

#[tokio::test]
async fn serialized_mathml_comment_is_removed_from_plain_and_markdown() {
    let chapter_xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <!-- MathML: <math><mi>x</mi></math> -->
    <p>Readable equation fallback: x squared.</p>
  </body>
</html>"#;

    for output_format in [OutputFormat::Plain, OutputFormat::Markdown] {
        let bytes = build_epub_bytes(chapter_xhtml);
        let extractor = EpubExtractor;
        let config = ExtractionConfig {
            output_format,
            ..Default::default()
        };
        let input = ExtractInput::from_bytes(bytes, "application/epub+zip", None);

        let result = extractor.extract(input, &config).await.expect("extract chapter");

        assert!(result.content.contains("Readable equation fallback"));
        assert!(!result.content.contains("MathML:"), "got: {}", result.content);
        assert!(!result.content.contains("<math"), "got: {}", result.content);
    }
}

#[tokio::test]
async fn unresolved_image_keeps_alt_text_and_caption_in_plain_and_markdown() {
    let chapter_xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <figure>
      <img src="missing.png" alt="Illustration alt text"/>
      <figcaption>Illustration caption</figcaption>
    </figure>
  </body>
</html>"#;

    for output_format in [OutputFormat::Plain, OutputFormat::Markdown] {
        let bytes = build_epub_bytes(chapter_xhtml);
        let extractor = EpubExtractor;
        let config = ExtractionConfig {
            output_format,
            ..Default::default()
        };
        let input = ExtractInput::from_bytes(bytes, "application/epub+zip", None);

        let result = extractor.extract(input, &config).await.expect("extract chapter");

        assert!(
            result.content.contains("Illustration alt text"),
            "got: {}",
            result.content
        );
        assert!(
            result.content.contains("Illustration caption"),
            "got: {}",
            result.content
        );
    }
}
