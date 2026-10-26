//! Golden Markdown for single PDF pages: any change to the native Markdown output of a page in
//! `test_documents/pdf/regression_pages/` fails here, naming each page and its first differing
//! line.
//!
//! The pages are cut from public-domain and CC-licensed documents whose sources and licences are
//! recorded in `test_documents`. Each page's expected output is committed as
//! `tests/fixtures/pdf/regression_pages/<stem>.md`. Regenerate only for an intentional output
//! change, then review the diff:
//! `XBERG_PDF_PAGES_GOLDEN_WRITE=1 cargo test -p xberg --features full,formula-recognition,bundle-tessdata-eng --test pdf_regression_pages`.
//! Snapshots are written with CI's feature set because some features change native Markdown output.
//!
//! The pages need `test_documents/`, which is bucket-fetched and absent from a bare checkout, so
//! the test passes with a notice without them: run `python3 test_documents/scripts/fetch_corpus.py`.

#![allow(clippy::print_stderr)] // ~keep: test/bench binaries print by design; org logging policy exempts tests
#![cfg(feature = "pdf")]

mod helpers;
use helpers::{extract_uri_document_blocking, get_test_documents_dir};
use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use xberg::{ExtractionConfig, OutputFormat};

const WRITE_VAR: &str = "XBERG_PDF_PAGES_GOLDEN_WRITE";
const TEST_COMMAND: &str =
    "cargo test -p xberg --features full,formula-recognition,bundle-tessdata-eng --test pdf_regression_pages";
/// CI's `RUST_MIN_STACK`: debug builds with the full feature set overflow the 2 MiB thread default.
const THREAD_STACK_BYTES: usize = 16 * 1024 * 1024;
const MAX_REPORTED: usize = 20;
const MAX_THREADS: usize = 8;

fn snapshot_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pdf/regression_pages")
}

/// The page PDFs sorted by path, or none when the directory is absent.
fn pages() -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(get_test_documents_dir().join("pdf/regression_pages")) else {
        return Vec::new();
    };
    let mut pages: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "pdf"))
        .collect();
    pages.sort();
    pages
}

fn stem(path: &Path) -> String {
    path.file_stem().unwrap_or_default().to_string_lossy().into_owned()
}

fn extract(page: &Path) -> Result<String, String> {
    let config = ExtractionConfig {
        disable_ocr: true,
        use_cache: false,
        enable_quality_processing: false,
        output_format: OutputFormat::Markdown,
        ..Default::default()
    };
    extract_uri_document_blocking(page, Some("application/pdf"), &config)
        .map(|document| document.content)
        .map_err(|e| format!("extraction failed: {e}"))
}

/// Contiguous chunks joined in spawn order keep the results in page order.
fn extract_all(pages: &[PathBuf]) -> Vec<Result<String, String>> {
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(MAX_THREADS);
    let chunk_len = pages.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = pages
            .chunks(chunk_len)
            .map(|chunk| {
                std::thread::Builder::new()
                    .stack_size(THREAD_STACK_BYTES)
                    .spawn_scoped(scope, move || {
                        chunk.iter().map(|page| extract(page)).collect::<Vec<_>>()
                    })
                    .expect("spawn extraction thread")
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("extraction thread panicked"))
            .collect()
    })
}

/// Splits on `\n` rather than `lines()` so a changed trailing newline is a difference too.
fn first_difference(expected: &str, actual: &str) -> String {
    let show = |line: Option<&str>| line.map_or_else(|| "<end of file>".to_string(), |line| format!("{line:?}"));
    let (mut expected_lines, mut actual_lines) = (expected.split('\n'), actual.split('\n'));
    let mut number = 1;
    loop {
        match (expected_lines.next(), actual_lines.next()) {
            (Some(e), Some(a)) if e == a => number += 1,
            (e, a) => return format!("line {number}: expected {}, actual {}", show(e), show(a)),
        }
    }
}

fn remove_stale_snapshots(dir: &Path, pages: &[PathBuf]) {
    let live: HashSet<String> = pages.iter().map(|page| stem(page)).collect();
    let entries = std::fs::read_dir(dir).unwrap_or_else(|e| panic!("read {dir:?}: {e}"));
    for path in entries.filter_map(|entry| entry.ok().map(|entry| entry.path())) {
        if path.extension().is_some_and(|ext| ext == "md") && !live.contains(&stem(&path)) {
            std::fs::remove_file(&path).unwrap_or_else(|e| panic!("remove {path:?}: {e}"));
        }
    }
}

#[test]
fn regression_pages_match_their_markdown_snapshots() {
    let pages = pages();
    if pages.is_empty() {
        eprintln!("Skipping: no PDFs in test_documents/pdf/regression_pages (see fetch_corpus.py)");
        return;
    }
    let dir = snapshot_dir();
    let write = std::env::var_os(WRITE_VAR).is_some();
    if write {
        std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("create {dir:?}: {e}"));
    }

    let mut failures = Vec::new();
    for (page, output) in pages.iter().zip(extract_all(&pages)) {
        let name = stem(page);
        let path = dir.join(format!("{name}.md"));
        let failure = match output {
            Err(error) => Some(error),
            Ok(actual) if write => {
                std::fs::write(&path, actual).unwrap_or_else(|e| panic!("write {path:?}: {e}"));
                None
            }
            Ok(actual) => match std::fs::read_to_string(&path) {
                Err(_) => Some(format!("no snapshot at {}", path.display())),
                Ok(expected) if expected == actual => None,
                Ok(expected) => Some(first_difference(&expected, &actual)),
            },
        };
        failures.extend(failure.map(|failure| format!("{name}: {failure}")));
    }
    if write {
        remove_stale_snapshots(&dir, &pages);
    }
    if failures.is_empty() {
        return;
    }

    let mut message = format!("{} of {} regression pages failed:\n", failures.len(), pages.len());
    for failure in failures.iter().take(MAX_REPORTED) {
        writeln!(message, "  {failure}").unwrap();
    }
    if failures.len() > MAX_REPORTED {
        writeln!(message, "  ... and {} more", failures.len() - MAX_REPORTED).unwrap();
    }
    writeln!(message, "If the change is intended, regenerate and review the diff:").unwrap();
    write!(message, "  {WRITE_VAR}=1 {TEST_COMMAND}").unwrap();
    panic!("{message}");
}
