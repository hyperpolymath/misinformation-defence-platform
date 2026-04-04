// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

//! End-to-end (E2E) tests for the disinfo-eval evaluation pipeline.
//!
//! These tests exercise the full stack: dataset loading → model training →
//! detection → explanation → metrics.  They are intentionally coarse-grained;
//! use unit tests for fine-grained behaviour.

use disinfo_eval::baselines::{
    all_baselines, BaselineModel, KeywordBaseline, TfIdfBaseline,
};
use disinfo_eval::datasets::{Dataset, Label, Sample};
use disinfo_eval::explainability::ExplainabilityMetrics;
use disinfo_eval::metrics::{ConfusionMatrix, EvaluationMetrics};
use disinfo_eval::pipeline::{EvaluationConfig, EvaluationPipeline};
use std::collections::HashMap;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_sample(id: &str, text: &str, label: Label) -> Sample {
    Sample {
        id: id.to_string(),
        text: text.to_string(),
        label,
        original_label: None,
        metadata: HashMap::new(),
    }
}

// ── E2E Test 1: Full single-text pipeline ────────────────────────────────────

/// Verify the full single-text path:
///   text → keyword detection → explanation → confidence in range.
#[test]
fn e2e_single_text_pipeline() {
    let training = Dataset::load_synthetic(200, 7);

    let mut model = KeywordBaseline::new();
    model.train(&training.train);

    let input_text = "BREAKING exclusive shocking revelation doctors hate miracle cure";
    let sample = make_sample("e2e1", input_text, Label::Uncertain);

    // Detect
    let pred = model.predict(&sample);
    assert!(
        pred.probability >= 0.0 && pred.probability <= 1.0,
        "Pipeline probability out of range: {}",
        pred.probability
    );

    // Explain
    let pred_exp = model.predict_with_explanation(&sample);
    assert!(!pred_exp.explanation.evidence.is_empty(), "No evidence generated for obvious disinformation text");
    assert!(!pred_exp.explanation.summary.is_empty(), "Explanation summary is empty");

    // The text contains multiple disinformation keywords → must be flagged.
    assert_eq!(
        pred_exp.prediction.label,
        Label::Disinformation,
        "Expected Disinformation for highly sensationalist text"
    );
}

// ── E2E Test 2: Batch processing — all results valid ─────────────────────────

/// Feed a batch of 20 synthetic samples through TF-IDF, verify every
/// returned prediction is well-formed.
#[test]
fn e2e_batch_processing_all_valid() {
    let dataset = Dataset::load_synthetic(100, 42);

    let mut model = TfIdfBaseline::new();
    model.train(&dataset.train);

    let batch = &dataset.test;
    let predictions = model.predict_batch(batch);

    assert_eq!(
        predictions.len(),
        batch.len(),
        "Batch output length mismatch"
    );

    for (i, pred) in predictions.iter().enumerate() {
        assert!(
            pred.probability >= 0.0 && pred.probability <= 1.0,
            "Batch prediction[{}] probability out of range: {}",
            i, pred.probability
        );
        // Label must be one of the two definite classes (not Uncertain).
        assert!(
            pred.label == Label::Disinformation || pred.label == Label::Authentic,
            "Batch prediction[{}] returned Uncertain label",
            i
        );
    }
}

// ── E2E Test 3: Known misinformation patterns are correctly flagged ───────────

/// A set of texts hand-crafted to contain strong disinformation signals.
/// The keyword model must classify all of them as Disinformation.
#[test]
fn e2e_known_misinformation_patterns_flagged() {
    let strong_disinfo_texts = vec![
        "BREAKING: Shocking secret government conspiracy exposed exclusively",
        "You won't believe this miracle cure that doctors hate and want hidden",
        "Urgent — share now before it's deleted: terrifying coverup revealed",
        "Outraged citizens expose horrifying hidden truth about secret plan",
        "Unbelievable viral truth: conspiracy confirmed shocking exclusive",
    ];

    let mut model = KeywordBaseline::new();
    // No training needed — keyword baseline uses a fixed list.
    let empty: Vec<Sample> = vec![];
    model.train(&empty);

    for text in &strong_disinfo_texts {
        let sample = make_sample("kd", text, Label::Uncertain);
        let pred = model.predict(&sample);
        assert_eq!(
            pred.label,
            Label::Disinformation,
            "Expected Disinformation for: {:?} (got {:?} with prob {:.3})",
            text, pred.label, pred.probability
        );
        assert!(
            pred.probability > 0.5,
            "Expected probability > 0.5 for '{}', got {:.3}",
            text, pred.probability
        );
    }
}

// ── E2E Test 4: Known authentic patterns are not flagged ─────────────────────

/// Texts containing strong authenticity signals must NOT be labelled Disinformation.
#[test]
fn e2e_authentic_patterns_not_flagged() {
    let authentic_texts = vec![
        "According to a peer-reviewed study published by the university institute",
        "Official spokesperson confirmed the verified research findings",
        "However, although experts say the evidence suggests otherwise",
        "The journal published data analysis confirming the study results",
    ];

    let mut model = KeywordBaseline::new();
    let empty: Vec<Sample> = vec![];
    model.train(&empty);

    for text in &authentic_texts {
        let sample = make_sample("auth", text, Label::Uncertain);
        let pred = model.predict(&sample);
        assert_eq!(
            pred.label,
            Label::Authentic,
            "Expected Authentic for '{}' (got {:?})",
            text, pred.label
        );
    }
}

// ── E2E Test 5: Full evaluation pipeline with metrics ────────────────────────

/// Run the EvaluationPipeline end-to-end on a synthetic dataset; verify all
/// aggregate metrics are in valid ranges.
#[test]
fn e2e_evaluation_pipeline_metrics_valid() {
    let config = EvaluationConfig {
        seed: 42,
        dataset_id: "synthetic".to_string(),
        dataset_path: None,
        eval_split: "test".to_string(),
        run_baselines: true,
        baseline_names: vec!["Keyword".to_string(), "Majority".to_string()],
        output_dir: "/tmp/disinfo_eval_e2e_test".to_string(),
    };

    let mut pipeline = EvaluationPipeline::new(config);
    let results = pipeline.run().expect("Pipeline must not error");

    assert_eq!(results.baseline_results.len(), 2, "Expected 2 baseline results");

    for result in &results.baseline_results {
        let cls = &result.metrics.classification;

        assert!(cls.accuracy >= 0.0 && cls.accuracy <= 1.0, "{}: accuracy out of range", result.model_name);
        assert!(cls.precision >= 0.0 && cls.precision <= 1.0, "{}: precision out of range", result.model_name);
        assert!(cls.recall >= 0.0 && cls.recall <= 1.0, "{}: recall out of range", result.model_name);
        assert!(cls.f1_score >= 0.0 && cls.f1_score <= 1.0, "{}: f1 out of range", result.model_name);
        assert!(cls.mcc >= -1.0 && cls.mcc <= 1.0, "{}: MCC out of range", result.model_name);

        if let Some(auc) = result.metrics.auc_roc {
            assert!(auc >= 0.0 && auc <= 1.0, "{}: AUC-ROC out of range: {}", result.model_name, auc);
        }
    }

    assert!(!results.summary.best_model.is_empty(), "Best model name must not be empty");
}

// ── E2E Test 6: Explainability metrics for TF-IDF ────────────────────────────

/// The TF-IDF model supports explanations; its explainability metrics must be
/// generated and internally consistent.
#[test]
fn e2e_tfidf_explainability_metrics() {
    let config = EvaluationConfig {
        seed: 42,
        dataset_id: "synthetic".to_string(),
        dataset_path: None,
        eval_split: "test".to_string(),
        run_baselines: true,
        baseline_names: vec!["TF-IDF".to_string()],
        output_dir: "/tmp/disinfo_eval_e2e_test".to_string(),
    };

    let mut pipeline = EvaluationPipeline::new(config);
    let results = pipeline.run().expect("Pipeline must not error");

    let tfidf_result = results.baseline_results.iter()
        .find(|r| r.model_name == "TF-IDF")
        .expect("TF-IDF result must be present");

    assert!(tfidf_result.supports_explanations, "TF-IDF must support explanations");

    let exp_metrics = tfidf_result.explainability_metrics.as_ref()
        .expect("TF-IDF must have explainability metrics");

    assert!(
        exp_metrics.avg_completeness >= 0.0 && exp_metrics.avg_completeness <= 1.0,
        "avg_completeness out of range: {}", exp_metrics.avg_completeness
    );
    assert!(
        exp_metrics.avg_evidence_count >= 0.0,
        "avg_evidence_count must be non-negative"
    );
    assert!(
        exp_metrics.calibration_error >= 0.0 && exp_metrics.calibration_error <= 1.0,
        "calibration_error out of range: {}", exp_metrics.calibration_error
    );
}

// ── E2E Test 7: Report generation is non-empty and well-formed ───────────────

#[test]
fn e2e_report_generation_well_formed() {
    let config = EvaluationConfig::default();
    let mut pipeline = EvaluationPipeline::new(config);
    let results = pipeline.run().expect("Pipeline must not error");

    let report = EvaluationPipeline::generate_report(&results);

    // Structural checks.
    assert!(report.contains("# Disinformation Detection Evaluation Report"), "Report missing title");
    assert!(report.contains("## Dataset"), "Report missing Dataset section");
    assert!(report.contains("## Summary"), "Report missing Summary section");
    assert!(report.contains("Baseline Comparison"), "Report missing comparison table");
    assert!(report.contains("## Detailed Results"), "Report missing detailed results");
    assert!(report.len() > 500, "Report suspiciously short: {} chars", report.len());
}

// ── E2E Test 8: Confusion matrix invariants ───────────────────────────────────

/// TP + TN + FP + FN must equal the total number of samples for any prediction set.
#[test]
fn e2e_confusion_matrix_invariant() {
    let dataset = Dataset::load_synthetic(200, 99);
    let mut model = TfIdfBaseline::new();
    model.train(&dataset.train);
    let preds = model.predict_batch(&dataset.test);

    let pred_labels: Vec<Label> = preds.iter().map(|p| p.label).collect();
    let true_labels: Vec<Label> = dataset.test.iter().map(|s| s.label).collect();

    let cm = ConfusionMatrix::from_predictions(&pred_labels, &true_labels);
    let total = cm.tp + cm.tn + cm.fp + cm.fn_;

    assert_eq!(
        total,
        dataset.test.len(),
        "Confusion matrix total ({}) != test set size ({})",
        total, dataset.test.len()
    );
}
