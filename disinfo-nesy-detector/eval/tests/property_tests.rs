// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

//! Property-based (P2P) tests for the disinfo-eval pipeline.
//!
//! These tests verify invariants that must hold for ALL inputs, not just
//! the hand-crafted examples in unit tests.
//!
//! Properties covered:
//! - Confidence scores are always in [0.0, 1.0]
//! - Keyword-based and majority detections are deterministic
//! - Non-empty explanations always have at least one evidence item when
//!   the model supports explanations and keywords are matched
//! - Label round-trips through binary representation are stable

use disinfo_eval::baselines::{
    KeywordBaseline, MajorityBaseline, RandomBaseline, StratifiedBaseline, TfIdfBaseline,
    BaselineModel,
};
use disinfo_eval::datasets::{Dataset, Label, Sample};
use disinfo_eval::explainability::ExplanationBuilder;
use proptest::prelude::*;
use std::collections::HashMap;

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Build a minimal labelled sample from a string slice.
fn make_sample(id: &str, text: &str, label: Label) -> Sample {
    Sample {
        id: id.to_string(),
        text: text.to_string(),
        label,
        original_label: None,
        metadata: HashMap::new(),
    }
}

/// A small balanced training set used across multiple properties.
fn small_training_set() -> Vec<Sample> {
    vec![
        make_sample("tr1", "BREAKING exclusive shocking doctors hate miracle cure", Label::Disinformation),
        make_sample("tr2", "According to peer-reviewed study published by university institute", Label::Authentic),
        make_sample("tr3", "Conspiracy secret hidden coverup viral share now urgent", Label::Disinformation),
        make_sample("tr4", "Research confirmed by official spokesperson verified evidence", Label::Authentic),
        make_sample("tr5", "Unbelievable outraged horrifying terrifying secret exposed", Label::Disinformation),
        make_sample("tr6", "Study shows experts say however although evidence suggests", Label::Authentic),
    ]
}

// ── Strategy for arbitrary text inputs ──────────────────────────────────────

/// Proptest strategy that generates a wide variety of text strings including
/// ASCII, Unicode, whitespace-only, empty, and very long inputs.
fn arb_text() -> impl Strategy<Value = String> {
    prop_oneof![
        // Standard ASCII words
        "[a-zA-Z0-9 ,.!?]{0,200}",
        // Unicode characters including CJK, emoji, combining marks
        ".{0,100}",
        // Whitespace-only
        "[ \t\n\r]{0,20}",
        // Very long (stress)
        "[a-z ]{500,2000}",
        // Mixed sensational + neutral
        Just("BREAKING shocking miracle cure doctors hate".to_string()),
        Just("according to peer-reviewed research published journal".to_string()),
        Just("".to_string()),
    ]
}

// ── Property 1: Confidence is always in [0.0, 1.0] ─────────────────────────

proptest! {
    /// For any text, the KeywordBaseline probability must stay within [0.0, 1.0].
    #[test]
    fn prop_keyword_confidence_in_range(text in arb_text()) {
        let mut model = KeywordBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred = model.predict(&sample);
        prop_assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "KeywordBaseline probability {} out of [0,1] for text: {:?}",
            pred.probability, text
        );
    }

    /// MajorityBaseline probability must stay within [0.0, 1.0] after training.
    #[test]
    fn prop_majority_confidence_in_range(text in arb_text()) {
        let mut model = MajorityBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred = model.predict(&sample);
        prop_assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "MajorityBaseline probability {} out of [0,1]", pred.probability
        );
    }

    /// StratifiedBaseline probability must stay within [0.0, 1.0].
    #[test]
    fn prop_stratified_confidence_in_range(text in arb_text()) {
        let mut model = StratifiedBaseline::new(42);
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred = model.predict(&sample);
        prop_assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "StratifiedBaseline probability {} out of [0,1]", pred.probability
        );
    }

    /// TfIdfBaseline probability must stay within [0.0, 1.0].
    #[test]
    fn prop_tfidf_confidence_in_range(text in arb_text()) {
        let mut model = TfIdfBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred = model.predict(&sample);
        prop_assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "TfIdfBaseline probability {} out of [0,1] for text: {:?}",
            pred.probability, text
        );
    }
}

// ── Property 2: Determinism ──────────────────────────────────────────────────

proptest! {
    /// KeywordBaseline is stateless after training, so the same input always
    /// produces the same output.
    #[test]
    fn prop_keyword_is_deterministic(text in arb_text()) {
        let mut model = KeywordBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred1 = model.predict(&sample);
        let pred2 = model.predict(&sample);
        prop_assert_eq!(
            format!("{:?}", pred1.label),
            format!("{:?}", pred2.label),
            "KeywordBaseline gave different labels on same input"
        );
        prop_assert!(
            (pred1.probability - pred2.probability).abs() < f64::EPSILON,
            "KeywordBaseline gave different probabilities on same input"
        );
    }

    /// MajorityBaseline always returns the same label for every input — it
    /// picks the majority class and never changes.
    #[test]
    fn prop_majority_is_constant(text1 in arb_text(), text2 in arb_text()) {
        let mut model = MajorityBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let s1 = make_sample("a", &text1, Label::Uncertain);
        let s2 = make_sample("b", &text2, Label::Uncertain);
        let p1 = model.predict(&s1);
        let p2 = model.predict(&s2);
        // Both must be the same label (majority class is constant).
        prop_assert_eq!(
            format!("{:?}", p1.label),
            format!("{:?}", p2.label),
            "MajorityBaseline produced different labels for different inputs"
        );
    }
}

// ── Property 3: Explanation length ≥ 1 for positive keyword detections ──────

proptest! {
    /// When KeywordBaseline detects at least one keyword (returns Disinformation),
    /// its explain-enabled prediction must include ≥ 1 evidence item.
    #[test]
    fn prop_keyword_explanation_nonempty_on_positive(
        // Always include at least one disinformation keyword.
        suffix in "[a-z ]{0,50}"
    ) {
        let text = format!("BREAKING shocking {}", suffix);
        let mut model = KeywordBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let sample = make_sample("p", &text, Label::Uncertain);
        let pred_with_exp = model.predict_with_explanation(&sample);
        // Keyword model explicitly supports explanations.
        prop_assert!(
            !pred_with_exp.explanation.evidence.is_empty(),
            "KeywordBaseline produced empty evidence for positive detection on: {:?}",
            text
        );
    }
}

// ── Property 4: Label binary round-trip ─────────────────────────────────────

proptest! {
    /// Label::Disinformation and Label::Authentic must survive a round-trip
    /// through to_binary() / from_binary().
    #[test]
    fn prop_label_binary_roundtrip(is_disinfo: bool) {
        let original = if is_disinfo { Label::Disinformation } else { Label::Authentic };
        let binary = original.to_binary().expect("Disinformation/Authentic always have binary form");
        let recovered = Label::from_binary(binary);
        prop_assert_eq!(
            format!("{:?}", original),
            format!("{:?}", recovered),
            "Label did not round-trip through binary representation"
        );
    }
}

// ── Property 5: Batch and single predictions are consistent ─────────────────

proptest! {
    /// predict_batch must return the same predictions as calling predict() one
    /// at a time for the TF-IDF model.
    #[test]
    fn prop_tfidf_batch_matches_single(texts in prop::collection::vec(arb_text(), 1..=10)) {
        let mut model = TfIdfBaseline::new();
        let training = small_training_set();
        model.train(&training);
        let samples: Vec<Sample> = texts.iter().enumerate()
            .map(|(i, t)| make_sample(&format!("p{}", i), t, Label::Uncertain))
            .collect();
        let batch_preds = model.predict_batch(&samples);
        for (i, sample) in samples.iter().enumerate() {
            let single_pred = model.predict(sample);
            prop_assert_eq!(
                format!("{:?}", batch_preds[i].label),
                format!("{:?}", single_pred.label),
                "TF-IDF batch prediction[{}] differs from single prediction", i
            );
            prop_assert!(
                (batch_preds[i].probability - single_pred.probability).abs() < 1e-9,
                "TF-IDF batch probability[{}] differs from single probability", i
            );
        }
    }
}

// ── Property 6: Dataset total_samples equals split sum ───────────────────────

proptest! {
    /// For any synthetic dataset size, the split sizes must add up correctly.
    #[test]
    fn prop_synthetic_split_invariant(size in 10usize..=500usize) {
        let dataset = Dataset::load_synthetic(size, 42);
        let total = dataset.train.len() + dataset.validation.len() + dataset.test.len();
        prop_assert_eq!(
            total, size,
            "Split sizes ({} + {} + {}) != total {}",
            dataset.train.len(), dataset.validation.len(), dataset.test.len(), size
        );
    }
}

// ── Property 7: ExplanationBuilder confidence is preserved ───────────────────

proptest! {
    /// The confidence passed to ExplanationBuilder must be preserved in the
    /// resulting Explanation.
    #[test]
    fn prop_explanation_confidence_preserved(confidence in 0.0f64..=1.0f64) {
        let explanation = ExplanationBuilder::new(Label::Disinformation, confidence).build();
        prop_assert!(
            (explanation.confidence - confidence).abs() < f64::EPSILON,
            "ExplanationBuilder did not preserve confidence {} (got {})",
            confidence, explanation.confidence
        );
    }
}
