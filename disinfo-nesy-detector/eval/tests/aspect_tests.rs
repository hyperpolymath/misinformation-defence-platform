// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

//! Aspect tests for the disinfo-eval pipeline.
//!
//! Aspect tests verify cross-cutting concerns that don't belong to a single
//! module:
//!   - Security: adversarial / malformed inputs must not panic or crash
//!   - Performance: single detection must complete within 100 ms
//!   - Error handling: empty, whitespace-only, and extremely long text
//!   - Robustness: Unicode, null bytes encoded as replacement chars, control chars

use disinfo_eval::baselines::{BaselineModel, KeywordBaseline, TfIdfBaseline};
use disinfo_eval::datasets::{Dataset, Label, Sample};
use std::collections::HashMap;
use std::time::Instant;

// ── Helper ────────────────────────────────────────────────────────────────────

fn make_sample(id: &str, text: &str, label: Label) -> Sample {
    Sample {
        id: id.to_string(),
        text: text.to_string(),
        label,
        original_label: None,
        metadata: HashMap::new(),
    }
}

fn trained_keyword() -> KeywordBaseline {
    let mut m = KeywordBaseline::new();
    let empty: Vec<Sample> = vec![];
    m.train(&empty);
    m
}

fn trained_tfidf() -> TfIdfBaseline {
    let dataset = Dataset::load_synthetic(200, 1);
    let mut m = TfIdfBaseline::new();
    m.train(&dataset.train);
    m
}

// ── Security: adversarial inputs ─────────────────────────────────────────────

/// Null bytes encoded in a Rust string (as U+0000) must not cause a panic.
#[test]
fn aspect_null_bytes_do_not_panic() {
    let model = trained_keyword();
    let text = "normal text\x00with null\x00bytes\x00inside";
    let sample = make_sample("null", text, Label::Uncertain);
    let pred = model.predict(&sample); // Must not panic.
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

/// An extremely long input (100 000 chars) must not cause a stack overflow
/// or OOM; the model must complete and return a valid probability.
#[test]
fn aspect_oversized_input_does_not_crash() {
    let model = trained_tfidf();
    let long_text = "disinformation ".repeat(6_667); // ~100 005 chars
    let sample = make_sample("long", &long_text, Label::Uncertain);
    let pred = model.predict(&sample);
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

/// Text consisting entirely of Unicode combining marks and zero-width joiners.
#[test]
fn aspect_unicode_combining_chars() {
    let model = trained_keyword();
    // U+0300 COMBINING GRAVE ACCENT, U+200D ZWJ, U+FEFF BOM
    let text = "\u{0300}\u{200D}\u{FEFF}\u{0301}\u{200C}";
    let sample = make_sample("uni", text, Label::Uncertain);
    let pred = model.predict(&sample);
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

/// Right-to-left override characters (potential homoglyph/bidi attack).
#[test]
fn aspect_bidi_override_chars() {
    let model = trained_keyword();
    let text = "\u{202E}BREAKING reverse text\u{202C}";
    let sample = make_sample("bidi", text, Label::Uncertain);
    let pred = model.predict(&sample);
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

/// Repeated long keyword — should not cause quadratic blowup.
#[test]
fn aspect_repeated_keyword_no_blowup() {
    let model = trained_tfidf();
    let text = "BREAKING ".repeat(5_000); // 45 000 chars
    let sample = make_sample("rep", &text, Label::Uncertain);
    let start = Instant::now();
    let pred = model.predict(&sample);
    let elapsed = start.elapsed();
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
    assert!(
        elapsed.as_secs() < 5,
        "Repeated keyword prediction took too long: {:?}",
        elapsed
    );
}

/// Input consisting only of non-ASCII digits and symbols.
#[test]
fn aspect_non_ascii_digits_and_symbols() {
    let model = trained_tfidf();
    let text = "①②③ ½ ¼ ¾ ℃ ℉ ∞ ≠ ≤ ≥ √ ∑ ∏ ∫ ∂ ∇ ⊕ ⊗";
    let sample = make_sample("sym", text, Label::Uncertain);
    let pred = model.predict(&sample);
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

// ── Error handling: edge-case inputs ─────────────────────────────────────────

/// Empty string must not panic; probability must be in range.
#[test]
fn aspect_empty_input_handled() {
    let km = trained_keyword();
    let tm = trained_tfidf();

    for (name, pred) in [
        ("keyword", km.predict(&make_sample("e", "", Label::Uncertain))),
        ("tfidf",   tm.predict(&make_sample("e", "", Label::Uncertain))),
    ] {
        assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "{}: empty input probability out of range: {}",
            name, pred.probability
        );
    }
}

/// Whitespace-only text must not panic.
#[test]
fn aspect_whitespace_only_handled() {
    let km = trained_keyword();
    let tm = trained_tfidf();
    let texts = ["   ", "\t", "\n\r\n", "   \t   \n   "];

    for text in &texts {
        for (name, pred) in [
            ("keyword", km.predict(&make_sample("ws", text, Label::Uncertain))),
            ("tfidf",   tm.predict(&make_sample("ws", text, Label::Uncertain))),
        ] {
            assert!(
                pred.probability >= 0.0 && pred.probability <= 1.0,
                "{}: whitespace input probability out of range",
                name
            );
        }
    }
}

/// Exactly one ASCII character must not panic.
#[test]
fn aspect_single_char_input() {
    let model = trained_tfidf();
    for ch in &["a", "z", "0", "!", " "] {
        let pred = model.predict(&make_sample("sc", ch, Label::Uncertain));
        assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
    }
}

// ── Performance: detection under 100 ms ──────────────────────────────────────

/// A typical-length news article (~500 words / ~3 000 chars) must be classified
/// by the keyword model in under 100 ms.
#[test]
fn aspect_keyword_detection_under_100ms() {
    let model = trained_keyword();
    let article = "According to official reports the university spokesperson confirmed \
        research findings showing however that experts say although the data analysis \
        suggests peer-reviewed journal evidence has been published and verified by \
        the institute study results are consistent and trustworthy. "
        .repeat(20); // ~3 000 chars

    let sample = make_sample("perf", &article, Label::Uncertain);
    let start = Instant::now();
    let pred = model.predict(&sample);
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 100,
        "Keyword detection took {}ms (limit: 100ms)",
        elapsed.as_millis()
    );
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

/// TF-IDF single-article detection must complete within 100 ms after training.
#[test]
fn aspect_tfidf_detection_under_100ms() {
    let model = trained_tfidf();
    let article = "BREAKING shocking exclusive miracle cure doctors hate conspiracy \
        secret hidden exposed outraged horrifying terrifying share now viral \
        unbelievable urgent coverup revealed."
        .repeat(10); // ~1 500 chars

    let sample = make_sample("perf", &article, Label::Uncertain);
    let start = Instant::now();
    let pred = model.predict(&sample);
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 100,
        "TF-IDF detection took {}ms (limit: 100ms)",
        elapsed.as_millis()
    );
    assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
}

// ── Robustness: batch of mixed edge-cases ────────────────────────────────────

/// A mixed batch containing empty strings, whitespace, normal text, and Unicode
/// must complete without panic and return valid probabilities for all entries.
#[test]
fn aspect_mixed_batch_robustness() {
    let model = trained_tfidf();
    let texts = vec![
        "".to_string(),
        "   ".to_string(),
        "BREAKING shocking exclusive".to_string(),
        "according to peer-reviewed study".to_string(),
        "\u{0300}\u{200D}\u{FEFF}".to_string(),
        "a".to_string(),
        "x".repeat(10_000),
        "normal English sentence with standard vocabulary".to_string(),
    ];

    let samples: Vec<Sample> = texts.iter().enumerate()
        .map(|(i, t)| make_sample(&format!("mix{}", i), t, Label::Uncertain))
        .collect();

    let preds = model.predict_batch(&samples);

    assert_eq!(preds.len(), samples.len(), "Batch output length mismatch");
    for (i, pred) in preds.iter().enumerate() {
        assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "Mixed batch prediction[{}] probability out of range: {}",
            i, pred.probability
        );
    }
}

// ── Contract: label consistency ───────────────────────────────────────────────

/// Regardless of input, the returned label must be consistent with the probability:
/// - Disinformation ↔ probability > 0.5
/// - Authentic       ↔ probability ≤ 0.5
///
/// This holds for the keyword baseline by construction.
#[test]
fn aspect_label_probability_consistency_keyword() {
    let model = trained_keyword();
    let texts = vec![
        "BREAKING shocking miracle",
        "according to research published",
        "secret conspiracy viral exposed",
        "official spokesperson verified",
        "normal ordinary everyday text",
        "",
    ];

    for text in &texts {
        let sample = make_sample("con", text, Label::Uncertain);
        let pred = model.predict(&sample);
        match pred.label {
            Label::Disinformation => {
                assert!(
                    pred.probability > 0.5,
                    "Disinformation label with probability <= 0.5: {} for '{}'",
                    pred.probability, text
                );
            }
            Label::Authentic => {
                assert!(
                    pred.probability <= 0.5,
                    "Authentic label with probability > 0.5: {} for '{}'",
                    pred.probability, text
                );
            }
            Label::Uncertain => {
                // Keyword model must never return Uncertain.
                panic!("KeywordBaseline returned Uncertain label for '{}'", text);
            }
        }
    }
}
