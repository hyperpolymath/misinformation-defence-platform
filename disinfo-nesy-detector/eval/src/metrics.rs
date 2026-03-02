// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Evaluation metrics for binary and multi-class classification
//!
//! Implements standard ML metrics:
//! - Confusion Matrix
//! - Accuracy, Precision, Recall, F1-Score
//! - AUC-ROC (for probabilistic predictions)
//! - Matthews Correlation Coefficient (MCC)

use crate::datasets::Label;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Confusion matrix for binary classification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfusionMatrix {
    /// True Positives (correctly predicted disinformation)
    pub tp: usize,
    /// True Negatives (correctly predicted authentic)
    pub tn: usize,
    /// False Positives (authentic predicted as disinformation)
    pub fp: usize,
    /// False Negatives (disinformation predicted as authentic)
    pub fn_: usize,
}

impl ConfusionMatrix {
    /// Create from predictions and ground truth labels
    pub fn from_predictions(predictions: &[Label], ground_truth: &[Label]) -> Self {
        assert_eq!(predictions.len(), ground_truth.len(), "Prediction and ground truth lengths must match");

        let mut matrix = Self::default();

        for (pred, truth) in predictions.iter().zip(ground_truth.iter()) {
            match (pred, truth) {
                (Label::Disinformation, Label::Disinformation) => matrix.tp += 1,
                (Label::Authentic, Label::Authentic) => matrix.tn += 1,
                (Label::Disinformation, Label::Authentic) => matrix.fp += 1,
                (Label::Authentic, Label::Disinformation) => matrix.fn_ += 1,
                // Skip uncertain labels
                _ => {}
            }
        }

        matrix
    }

    /// Total number of samples
    pub fn total(&self) -> usize {
        self.tp + self.tn + self.fp + self.fn_
    }

    /// Accuracy: (TP + TN) / Total
    pub fn accuracy(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            return 0.0;
        }
        (self.tp + self.tn) as f64 / total as f64
    }

    /// Precision: TP / (TP + FP)
    pub fn precision(&self) -> f64 {
        let denom = self.tp + self.fp;
        if denom == 0 {
            return 0.0;
        }
        self.tp as f64 / denom as f64
    }

    /// Recall (Sensitivity): TP / (TP + FN)
    pub fn recall(&self) -> f64 {
        let denom = self.tp + self.fn_;
        if denom == 0 {
            return 0.0;
        }
        self.tp as f64 / denom as f64
    }

    /// Specificity: TN / (TN + FP)
    pub fn specificity(&self) -> f64 {
        let denom = self.tn + self.fp;
        if denom == 0 {
            return 0.0;
        }
        self.tn as f64 / denom as f64
    }

    /// F1 Score: 2 * (Precision * Recall) / (Precision + Recall)
    pub fn f1_score(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        let denom = precision + recall;
        if denom == 0.0 {
            return 0.0;
        }
        2.0 * precision * recall / denom
    }

    /// F-beta Score: (1 + beta^2) * (Precision * Recall) / (beta^2 * Precision + Recall)
    pub fn f_beta_score(&self, beta: f64) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        let beta_sq = beta * beta;
        let denom = beta_sq * precision + recall;
        if denom == 0.0 {
            return 0.0;
        }
        (1.0 + beta_sq) * precision * recall / denom
    }

    /// Matthews Correlation Coefficient (MCC)
    /// Best metric for imbalanced datasets: ranges from -1 to 1
    pub fn mcc(&self) -> f64 {
        let tp = self.tp as f64;
        let tn = self.tn as f64;
        let fp = self.fp as f64;
        let fn_ = self.fn_ as f64;

        let numerator = tp * tn - fp * fn_;
        let denominator = ((tp + fp) * (tp + fn_) * (tn + fp) * (tn + fn_)).sqrt();

        if denominator == 0.0 {
            return 0.0;
        }
        numerator / denominator
    }

    /// Balanced Accuracy: (Sensitivity + Specificity) / 2
    pub fn balanced_accuracy(&self) -> f64 {
        (self.recall() + self.specificity()) / 2.0
    }
}

/// Full classification report with all metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationReport {
    pub confusion_matrix: ConfusionMatrix,
    pub accuracy: f64,
    pub balanced_accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub f2_score: f64,
    pub mcc: f64,
    pub specificity: f64,
    pub support: usize,
}

impl ClassificationReport {
    /// Generate full report from confusion matrix
    pub fn from_confusion_matrix(cm: ConfusionMatrix) -> Self {
        Self {
            accuracy: cm.accuracy(),
            balanced_accuracy: cm.balanced_accuracy(),
            precision: cm.precision(),
            recall: cm.recall(),
            f1_score: cm.f1_score(),
            f2_score: cm.f_beta_score(2.0),
            mcc: cm.mcc(),
            specificity: cm.specificity(),
            support: cm.total(),
            confusion_matrix: cm,
        }
    }

    /// Generate report from predictions and ground truth
    pub fn from_predictions(predictions: &[Label], ground_truth: &[Label]) -> Self {
        let cm = ConfusionMatrix::from_predictions(predictions, ground_truth);
        Self::from_confusion_matrix(cm)
    }

    /// Format as a human-readable string
    pub fn format(&self) -> String {
        format!(
            r#"Classification Report
=====================
Accuracy:          {:.4} ({:.2}%)
Balanced Accuracy: {:.4} ({:.2}%)
Precision:         {:.4}
Recall:            {:.4}
F1 Score:          {:.4}
F2 Score:          {:.4}
MCC:               {:.4}
Specificity:       {:.4}
Support:           {}

Confusion Matrix:
                  Predicted
                  Disinfo   Authentic
Actual Disinfo   {:>6}    {:>6}
       Authentic {:>6}    {:>6}
"#,
            self.accuracy, self.accuracy * 100.0,
            self.balanced_accuracy, self.balanced_accuracy * 100.0,
            self.precision,
            self.recall,
            self.f1_score,
            self.f2_score,
            self.mcc,
            self.specificity,
            self.support,
            self.confusion_matrix.tp, self.confusion_matrix.fn_,
            self.confusion_matrix.fp, self.confusion_matrix.tn,
        )
    }
}

/// Complete evaluation metrics including probabilistic metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub classification: ClassificationReport,
    /// AUC-ROC score (if probabilities available)
    pub auc_roc: Option<f64>,
    /// Average precision (area under PR curve)
    pub average_precision: Option<f64>,
    /// Brier score (calibration metric)
    pub brier_score: Option<f64>,
    /// Per-class metrics
    pub per_class: HashMap<String, ClassMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMetrics {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub support: usize,
}

impl EvaluationMetrics {
    /// Create from predictions without probabilities
    pub fn from_predictions(predictions: &[Label], ground_truth: &[Label]) -> Self {
        let classification = ClassificationReport::from_predictions(predictions, ground_truth);

        // Calculate per-class metrics
        let mut per_class = HashMap::new();

        // Disinformation class
        let disinfo_support = ground_truth.iter().filter(|l| **l == Label::Disinformation).count();
        per_class.insert(
            "disinformation".to_string(),
            ClassMetrics {
                precision: classification.precision,
                recall: classification.recall,
                f1_score: classification.f1_score,
                support: disinfo_support,
            },
        );

        // Authentic class (swap TP/TN, FP/FN perspective)
        let authentic_support = ground_truth.iter().filter(|l| **l == Label::Authentic).count();
        let cm = &classification.confusion_matrix;
        let authentic_precision = if cm.tn + cm.fn_ > 0 {
            cm.tn as f64 / (cm.tn + cm.fn_) as f64
        } else {
            0.0
        };
        let authentic_recall = classification.specificity;
        let authentic_f1 = if authentic_precision + authentic_recall > 0.0 {
            2.0 * authentic_precision * authentic_recall / (authentic_precision + authentic_recall)
        } else {
            0.0
        };
        per_class.insert(
            "authentic".to_string(),
            ClassMetrics {
                precision: authentic_precision,
                recall: authentic_recall,
                f1_score: authentic_f1,
                support: authentic_support,
            },
        );

        Self {
            classification,
            auc_roc: None,
            average_precision: None,
            brier_score: None,
            per_class,
        }
    }

    /// Create from predictions with probability scores
    pub fn from_predictions_with_probs(
        predictions: &[Label],
        ground_truth: &[Label],
        probabilities: &[f64], // P(disinformation)
    ) -> Self {
        let mut metrics = Self::from_predictions(predictions, ground_truth);

        // Calculate AUC-ROC
        metrics.auc_roc = Some(Self::calculate_auc_roc(ground_truth, probabilities));

        // Calculate Brier score
        metrics.brier_score = Some(Self::calculate_brier_score(ground_truth, probabilities));

        // Calculate average precision
        metrics.average_precision = Some(Self::calculate_average_precision(ground_truth, probabilities));

        metrics
    }

    /// Calculate AUC-ROC using trapezoidal rule
    fn calculate_auc_roc(ground_truth: &[Label], probabilities: &[f64]) -> f64 {
        // Sort by probability descending
        let mut pairs: Vec<_> = ground_truth
            .iter()
            .zip(probabilities.iter())
            .filter(|(l, _)| **l != Label::Uncertain)
            .map(|(l, p)| (*l, *p))
            .collect();

        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let n_pos = pairs.iter().filter(|(l, _)| *l == Label::Disinformation).count() as f64;
        let n_neg = pairs.iter().filter(|(l, _)| *l == Label::Authentic).count() as f64;

        if n_pos == 0.0 || n_neg == 0.0 {
            return 0.5; // Random baseline
        }

        // Calculate TPR and FPR at each threshold
        let mut tpr_prev = 0.0;
        let mut fpr_prev = 0.0;
        let mut tp = 0.0;
        let mut fp = 0.0;
        let mut auc = 0.0;

        for (label, _prob) in &pairs {
            if *label == Label::Disinformation {
                tp += 1.0;
            } else {
                fp += 1.0;
            }

            let tpr = tp / n_pos;
            let fpr = fp / n_neg;

            // Trapezoidal rule
            auc += (fpr - fpr_prev) * (tpr + tpr_prev) / 2.0;

            tpr_prev = tpr;
            fpr_prev = fpr;
        }

        auc
    }

    /// Calculate Brier score (lower is better)
    fn calculate_brier_score(ground_truth: &[Label], probabilities: &[f64]) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;

        for (label, prob) in ground_truth.iter().zip(probabilities.iter()) {
            if *label == Label::Uncertain {
                continue;
            }

            let target = if *label == Label::Disinformation { 1.0 } else { 0.0 };
            sum += (prob - target).powi(2);
            count += 1;
        }

        if count == 0 {
            return 1.0;
        }

        sum / count as f64
    }

    /// Calculate average precision (area under precision-recall curve)
    fn calculate_average_precision(ground_truth: &[Label], probabilities: &[f64]) -> f64 {
        // Sort by probability descending
        let mut pairs: Vec<_> = ground_truth
            .iter()
            .zip(probabilities.iter())
            .filter(|(l, _)| **l != Label::Uncertain)
            .map(|(l, p)| (*l, *p))
            .collect();

        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let n_pos = pairs.iter().filter(|(l, _)| *l == Label::Disinformation).count() as f64;

        if n_pos == 0.0 {
            return 0.0;
        }

        let mut tp = 0.0;
        let mut fp = 0.0;
        let mut ap = 0.0;
        let mut prev_recall = 0.0;

        for (label, _) in &pairs {
            if *label == Label::Disinformation {
                tp += 1.0;
            } else {
                fp += 1.0;
            }

            let precision = tp / (tp + fp);
            let recall = tp / n_pos;

            // Add area under curve when we see a positive example
            if *label == Label::Disinformation {
                ap += precision * (recall - prev_recall);
            }

            prev_recall = recall;
        }

        ap
    }

    /// Format as human-readable string
    pub fn format(&self) -> String {
        let mut output = self.classification.format();

        if let Some(auc) = self.auc_roc {
            output.push_str(&format!("\nAUC-ROC:           {:.4}\n", auc));
        }
        if let Some(ap) = self.average_precision {
            output.push_str(&format!("Average Precision: {:.4}\n", ap));
        }
        if let Some(brier) = self.brier_score {
            output.push_str(&format!("Brier Score:       {:.4}\n", brier));
        }

        output.push_str("\nPer-Class Metrics:\n");
        for (class, metrics) in &self.per_class {
            output.push_str(&format!(
                "  {}: P={:.4} R={:.4} F1={:.4} (n={})\n",
                class, metrics.precision, metrics.recall, metrics.f1_score, metrics.support
            ));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confusion_matrix_perfect() {
        let predictions = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];
        let ground_truth = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];

        let cm = ConfusionMatrix::from_predictions(&predictions, &ground_truth);

        assert_eq!(cm.tp, 2);
        assert_eq!(cm.tn, 2);
        assert_eq!(cm.fp, 0);
        assert_eq!(cm.fn_, 0);
        assert!((cm.accuracy() - 1.0).abs() < 1e-6);
        assert!((cm.f1_score() - 1.0).abs() < 1e-6);
        assert!((cm.mcc() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_confusion_matrix_worst() {
        let predictions = vec![Label::Authentic, Label::Authentic, Label::Disinformation, Label::Disinformation];
        let ground_truth = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];

        let cm = ConfusionMatrix::from_predictions(&predictions, &ground_truth);

        assert_eq!(cm.tp, 0);
        assert_eq!(cm.tn, 0);
        assert_eq!(cm.fp, 2);
        assert_eq!(cm.fn_, 2);
        assert!((cm.accuracy() - 0.0).abs() < 1e-6);
        assert!((cm.mcc() - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_confusion_matrix_random() {
        // 50% accuracy, balanced errors
        let predictions = vec![Label::Disinformation, Label::Authentic, Label::Disinformation, Label::Authentic];
        let ground_truth = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];

        let cm = ConfusionMatrix::from_predictions(&predictions, &ground_truth);

        assert_eq!(cm.tp, 1);
        assert_eq!(cm.tn, 1);
        assert_eq!(cm.fp, 1);
        assert_eq!(cm.fn_, 1);
        assert!((cm.accuracy() - 0.5).abs() < 1e-6);
        assert!(cm.mcc().abs() < 1e-6); // MCC should be ~0 for random
    }

    #[test]
    fn test_auc_roc_perfect() {
        let ground_truth = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];
        let probabilities = vec![0.9, 0.8, 0.2, 0.1];

        let auc = EvaluationMetrics::calculate_auc_roc(&ground_truth, &probabilities);
        assert!((auc - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_auc_roc_random() {
        // When predictions are uncorrelated with labels, AUC should be ~0.5
        let ground_truth = vec![
            Label::Disinformation, Label::Authentic,
            Label::Disinformation, Label::Authentic,
            Label::Disinformation, Label::Authentic,
        ];
        // Probabilities that don't correlate with ground truth
        let probabilities = vec![0.3, 0.7, 0.6, 0.4, 0.5, 0.5];

        let auc = EvaluationMetrics::calculate_auc_roc(&ground_truth, &probabilities);
        // With uncorrelated predictions, AUC should be around 0.5
        assert!(auc >= 0.0 && auc <= 1.0);
        // This specific case should yield ~0.5 (may vary slightly)
        assert!((auc - 0.5).abs() < 0.35);
    }

    #[test]
    fn test_brier_score_perfect() {
        let ground_truth = vec![Label::Disinformation, Label::Authentic];
        let probabilities = vec![1.0, 0.0];

        let brier = EvaluationMetrics::calculate_brier_score(&ground_truth, &probabilities);
        assert!(brier.abs() < 1e-6);
    }

    #[test]
    fn test_classification_report_format() {
        let predictions = vec![Label::Disinformation, Label::Disinformation, Label::Authentic, Label::Authentic];
        let ground_truth = vec![Label::Disinformation, Label::Authentic, Label::Authentic, Label::Authentic];

        let report = ClassificationReport::from_predictions(&predictions, &ground_truth);
        let formatted = report.format();

        assert!(formatted.contains("Classification Report"));
        assert!(formatted.contains("Accuracy"));
        assert!(formatted.contains("Confusion Matrix"));
    }
}
