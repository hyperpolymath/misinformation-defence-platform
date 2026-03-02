// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Explainability module for disinformation detection
//!
//! Provides:
//! - Explanation structures for model decisions
//! - Feature attribution and importance
//! - Symbolic reasoning traces
//! - Explanation quality metrics

use crate::datasets::Label;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of evidence that can support a prediction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvidenceType {
    /// Linguistic pattern (e.g., sensationalist language)
    LinguisticPattern,
    /// Source credibility information
    SourceCredibility,
    /// Fact-check match from knowledge base
    FactCheck,
    /// Emotional manipulation indicator
    EmotionalManipulation,
    /// Visual artifact detection (for images)
    VisualArtifact,
    /// Propagation pattern (viral spread indicators)
    PropagationPattern,
    /// Symbolic rule fired
    SymbolicRule,
    /// Neural feature activation
    NeuralFeature,
    /// Keyword/term match
    KeywordMatch,
    /// Statistical anomaly
    StatisticalAnomaly,
}

/// A single piece of evidence supporting a prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Human-readable description
    pub description: String,
    /// Confidence/weight of this evidence (0.0 to 1.0)
    pub weight: f64,
    /// Text span or location (if applicable)
    pub span: Option<TextSpan>,
    /// Source of the evidence (e.g., rule name, feature name)
    pub source: String,
}

/// A span of text that is relevant to the explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSpan {
    /// Start character offset
    pub start: usize,
    /// End character offset
    pub end: usize,
    /// The actual text
    pub text: String,
}

/// A symbolic reasoning step in the explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Rule or inference that was applied
    pub rule: String,
    /// Premises that led to this conclusion
    pub premises: Vec<String>,
    /// Conclusion drawn
    pub conclusion: String,
    /// Confidence in this step
    pub confidence: f64,
}

/// Complete explanation for a prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// The predicted label
    pub prediction: Label,
    /// Confidence in the prediction
    pub confidence: f64,
    /// Evidence supporting the prediction
    pub evidence: Vec<Evidence>,
    /// Symbolic reasoning trace (for neuro-symbolic models)
    pub reasoning_trace: Vec<ReasoningStep>,
    /// Feature attributions (feature name -> attribution score)
    pub feature_attributions: HashMap<String, f64>,
    /// Counter-evidence (evidence against the prediction)
    pub counter_evidence: Vec<Evidence>,
    /// Natural language summary
    pub summary: String,
    /// Uncertainty factors
    pub uncertainty_factors: Vec<String>,
}

impl Default for Explanation {
    fn default() -> Self {
        Self {
            prediction: Label::Uncertain,
            confidence: 0.0,
            evidence: Vec::new(),
            reasoning_trace: Vec::new(),
            feature_attributions: HashMap::new(),
            counter_evidence: Vec::new(),
            summary: String::new(),
            uncertainty_factors: Vec::new(),
        }
    }
}

impl Explanation {
    /// Create a new explanation for a prediction
    pub fn new(prediction: Label, confidence: f64) -> Self {
        Self {
            prediction,
            confidence,
            ..Default::default()
        }
    }

    /// Add evidence to the explanation
    pub fn add_evidence(&mut self, evidence: Evidence) {
        self.evidence.push(evidence);
    }

    /// Add a reasoning step
    pub fn add_reasoning_step(&mut self, step: ReasoningStep) {
        self.reasoning_trace.push(step);
    }

    /// Set feature attributions
    pub fn set_attributions(&mut self, attributions: HashMap<String, f64>) {
        self.feature_attributions = attributions;
    }

    /// Generate a natural language summary
    pub fn generate_summary(&mut self) {
        let mut summary_parts = Vec::new();

        // Start with the verdict
        let verdict = match self.prediction {
            Label::Disinformation => "likely disinformation",
            Label::Authentic => "likely authentic",
            Label::Uncertain => "uncertain classification",
        };
        summary_parts.push(format!(
            "This content is classified as {} (confidence: {:.1}%).",
            verdict,
            self.confidence * 100.0
        ));

        // Summarize top evidence
        if !self.evidence.is_empty() {
            let mut sorted_evidence = self.evidence.clone();
            sorted_evidence.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

            let top_evidence: Vec<_> = sorted_evidence.iter().take(3).collect();
            if !top_evidence.is_empty() {
                summary_parts.push("Key indicators:".to_string());
                for ev in top_evidence {
                    summary_parts.push(format!("  • {}", ev.description));
                }
            }
        }

        // Add uncertainty factors
        if !self.uncertainty_factors.is_empty() {
            summary_parts.push("Uncertainty factors:".to_string());
            for factor in &self.uncertainty_factors {
                summary_parts.push(format!("  • {}", factor));
            }
        }

        self.summary = summary_parts.join("\n");
    }

    /// Get the top N most important features
    pub fn top_features(&self, n: usize) -> Vec<(&String, &f64)> {
        let mut features: Vec<_> = self.feature_attributions.iter().collect();
        features.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        features.into_iter().take(n).collect()
    }

    /// Calculate explanation completeness score
    pub fn completeness_score(&self) -> f64 {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Has evidence
        max_score += 1.0;
        if !self.evidence.is_empty() {
            score += 1.0;
        }

        // Has feature attributions
        max_score += 1.0;
        if !self.feature_attributions.is_empty() {
            score += 1.0;
        }

        // Has reasoning trace (for symbolic)
        max_score += 1.0;
        if !self.reasoning_trace.is_empty() {
            score += 1.0;
        }

        // Has summary
        max_score += 1.0;
        if !self.summary.is_empty() {
            score += 1.0;
        }

        // Acknowledges uncertainty
        max_score += 1.0;
        if !self.uncertainty_factors.is_empty() || self.confidence < 0.9 {
            score += 0.5; // Partial credit for low confidence
        }
        if !self.uncertainty_factors.is_empty() {
            score += 0.5;
        }

        score / max_score
    }
}

/// Metrics for evaluating explanation quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityMetrics {
    /// Average number of evidence pieces per explanation
    pub avg_evidence_count: f64,
    /// Average number of features with attributions
    pub avg_attribution_count: f64,
    /// Fraction of explanations with reasoning traces
    pub reasoning_trace_coverage: f64,
    /// Average explanation completeness score
    pub avg_completeness: f64,
    /// Consistency: how often top features align with prediction
    pub feature_consistency: f64,
    /// Average confidence calibration error
    pub calibration_error: f64,
    /// Fraction of explanations with uncertainty acknowledgment
    pub uncertainty_coverage: f64,
}

impl Default for ExplainabilityMetrics {
    fn default() -> Self {
        Self {
            avg_evidence_count: 0.0,
            avg_attribution_count: 0.0,
            reasoning_trace_coverage: 0.0,
            avg_completeness: 0.0,
            feature_consistency: 0.0,
            calibration_error: 0.0,
            uncertainty_coverage: 0.0,
        }
    }
}

impl ExplainabilityMetrics {
    /// Calculate metrics from a set of explanations and ground truth
    pub fn from_explanations(
        explanations: &[Explanation],
        ground_truth: &[Label],
        predicted_probs: &[f64],
    ) -> Self {
        if explanations.is_empty() {
            return Self::default();
        }

        let n = explanations.len() as f64;

        // Average evidence count
        let avg_evidence_count = explanations.iter().map(|e| e.evidence.len() as f64).sum::<f64>() / n;

        // Average attribution count
        let avg_attribution_count = explanations
            .iter()
            .map(|e| e.feature_attributions.len() as f64)
            .sum::<f64>()
            / n;

        // Reasoning trace coverage
        let reasoning_trace_coverage =
            explanations.iter().filter(|e| !e.reasoning_trace.is_empty()).count() as f64 / n;

        // Average completeness
        let avg_completeness = explanations.iter().map(|e| e.completeness_score()).sum::<f64>() / n;

        // Uncertainty coverage
        let uncertainty_coverage = explanations
            .iter()
            .filter(|e| !e.uncertainty_factors.is_empty())
            .count() as f64
            / n;

        // Feature consistency (simplified: check if high-weight evidence aligns with prediction)
        let mut consistent_count = 0;
        for exp in explanations {
            if exp.evidence.is_empty() {
                continue;
            }
            let top_evidence = exp.evidence.iter().max_by(|a, b| {
                a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal)
            });
            if let Some(ev) = top_evidence {
                // Check if evidence type aligns with prediction
                let evidence_suggests_disinfo = matches!(
                    ev.evidence_type,
                    EvidenceType::LinguisticPattern
                        | EvidenceType::EmotionalManipulation
                        | EvidenceType::VisualArtifact
                        | EvidenceType::PropagationPattern
                );
                let prediction_is_disinfo = exp.prediction == Label::Disinformation;
                if evidence_suggests_disinfo == prediction_is_disinfo {
                    consistent_count += 1;
                }
            }
        }
        let feature_consistency = if explanations.iter().any(|e| !e.evidence.is_empty()) {
            consistent_count as f64 / explanations.iter().filter(|e| !e.evidence.is_empty()).count() as f64
        } else {
            0.0
        };

        // Calibration error (Expected Calibration Error - simplified)
        let calibration_error = Self::calculate_calibration_error(explanations, ground_truth, predicted_probs);

        Self {
            avg_evidence_count,
            avg_attribution_count,
            reasoning_trace_coverage,
            avg_completeness,
            feature_consistency,
            calibration_error,
            uncertainty_coverage,
        }
    }

    /// Calculate Expected Calibration Error
    fn calculate_calibration_error(
        explanations: &[Explanation],
        ground_truth: &[Label],
        predicted_probs: &[f64],
    ) -> f64 {
        if explanations.is_empty() || ground_truth.len() != predicted_probs.len() {
            return 0.0;
        }

        // Bin predictions by confidence
        let n_bins = 10;
        let mut bin_correct = vec![0.0; n_bins];
        let mut bin_confidence = vec![0.0; n_bins];
        let mut bin_count = vec![0usize; n_bins];

        for (i, (exp, &truth)) in explanations.iter().zip(ground_truth.iter()).enumerate() {
            if i >= predicted_probs.len() {
                break;
            }
            let prob = predicted_probs[i];
            let bin = ((prob * n_bins as f64) as usize).min(n_bins - 1);

            bin_count[bin] += 1;
            bin_confidence[bin] += prob;
            if exp.prediction == truth {
                bin_correct[bin] += 1.0;
            }
        }

        // Calculate ECE
        let total = explanations.len() as f64;
        let mut ece = 0.0;
        for bin in 0..n_bins {
            if bin_count[bin] > 0 {
                let avg_conf = bin_confidence[bin] / bin_count[bin] as f64;
                let accuracy = bin_correct[bin] / bin_count[bin] as f64;
                ece += (bin_count[bin] as f64 / total) * (avg_conf - accuracy).abs();
            }
        }

        ece
    }

    /// Format as human-readable string
    pub fn format(&self) -> String {
        format!(
            r#"Explainability Metrics
======================
Avg Evidence Count:       {:.2}
Avg Attribution Count:    {:.2}
Reasoning Trace Coverage: {:.2}%
Avg Completeness Score:   {:.2}%
Feature Consistency:      {:.2}%
Calibration Error (ECE):  {:.4}
Uncertainty Coverage:     {:.2}%
"#,
            self.avg_evidence_count,
            self.avg_attribution_count,
            self.reasoning_trace_coverage * 100.0,
            self.avg_completeness * 100.0,
            self.feature_consistency * 100.0,
            self.calibration_error,
            self.uncertainty_coverage * 100.0,
        )
    }
}

/// Trait for models that provide explanations
pub trait Explainable {
    /// Generate an explanation for a prediction
    fn explain(&self, text: &str, prediction: Label, confidence: f64) -> Explanation;
}

/// Builder for constructing explanations
pub struct ExplanationBuilder {
    explanation: Explanation,
}

impl ExplanationBuilder {
    pub fn new(prediction: Label, confidence: f64) -> Self {
        Self {
            explanation: Explanation::new(prediction, confidence),
        }
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.explanation.add_evidence(evidence);
        self
    }

    pub fn with_keyword_match(mut self, keyword: &str, weight: f64) -> Self {
        self.explanation.add_evidence(Evidence {
            evidence_type: EvidenceType::KeywordMatch,
            description: format!("Contains indicator keyword: '{}'", keyword),
            weight,
            span: None,
            source: "keyword_detector".to_string(),
        });
        self
    }

    pub fn with_linguistic_pattern(mut self, pattern: &str, weight: f64) -> Self {
        self.explanation.add_evidence(Evidence {
            evidence_type: EvidenceType::LinguisticPattern,
            description: format!("Linguistic pattern detected: {}", pattern),
            weight,
            span: None,
            source: "linguistic_analyzer".to_string(),
        });
        self
    }

    pub fn with_source_credibility(mut self, source: &str, credibility: f64) -> Self {
        let description = if credibility > 0.7 {
            format!("Source '{}' has high credibility rating", source)
        } else if credibility > 0.4 {
            format!("Source '{}' has mixed credibility rating", source)
        } else {
            format!("Source '{}' has low credibility rating", source)
        };
        self.explanation.add_evidence(Evidence {
            evidence_type: EvidenceType::SourceCredibility,
            description,
            weight: credibility,
            span: None,
            source: "source_checker".to_string(),
        });
        self
    }

    pub fn with_reasoning_step(mut self, rule: &str, premises: Vec<String>, conclusion: &str, confidence: f64) -> Self {
        self.explanation.add_reasoning_step(ReasoningStep {
            rule: rule.to_string(),
            premises,
            conclusion: conclusion.to_string(),
            confidence,
        });
        self
    }

    pub fn with_attributions(mut self, attributions: HashMap<String, f64>) -> Self {
        self.explanation.set_attributions(attributions);
        self
    }

    pub fn with_uncertainty(mut self, factor: &str) -> Self {
        self.explanation.uncertainty_factors.push(factor.to_string());
        self
    }

    pub fn build(mut self) -> Explanation {
        self.explanation.generate_summary();
        self.explanation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explanation_builder() {
        let explanation = ExplanationBuilder::new(Label::Disinformation, 0.85)
            .with_keyword_match("BREAKING", 0.7)
            .with_linguistic_pattern("sensationalist language", 0.8)
            .with_uncertainty("Limited context available")
            .build();

        assert_eq!(explanation.prediction, Label::Disinformation);
        assert_eq!(explanation.evidence.len(), 2);
        assert_eq!(explanation.uncertainty_factors.len(), 1);
        assert!(!explanation.summary.is_empty());
    }

    #[test]
    fn test_completeness_score() {
        let mut explanation = Explanation::new(Label::Disinformation, 0.9);

        // Empty explanation should have low completeness
        let score_empty = explanation.completeness_score();
        assert!(score_empty < 0.5);

        // Add components
        explanation.add_evidence(Evidence {
            evidence_type: EvidenceType::KeywordMatch,
            description: "Test".to_string(),
            weight: 0.5,
            span: None,
            source: "test".to_string(),
        });
        explanation.feature_attributions.insert("test_feature".to_string(), 0.8);
        explanation.uncertainty_factors.push("Test uncertainty".to_string());
        explanation.generate_summary();

        let score_full = explanation.completeness_score();
        assert!(score_full > score_empty);
    }

    #[test]
    fn test_explainability_metrics() {
        let explanations = vec![
            ExplanationBuilder::new(Label::Disinformation, 0.9)
                .with_keyword_match("SHOCKING", 0.8)
                .build(),
            ExplanationBuilder::new(Label::Authentic, 0.7)
                .with_source_credibility("reuters.com", 0.95)
                .build(),
        ];
        let ground_truth = vec![Label::Disinformation, Label::Authentic];
        let probs = vec![0.9, 0.3];

        let metrics = ExplainabilityMetrics::from_explanations(&explanations, &ground_truth, &probs);

        assert!(metrics.avg_evidence_count >= 1.0);
        assert!(metrics.avg_completeness > 0.0);
    }

    #[test]
    fn test_top_features() {
        let mut explanation = Explanation::new(Label::Disinformation, 0.8);
        explanation.feature_attributions.insert("feature_a".to_string(), 0.9);
        explanation.feature_attributions.insert("feature_b".to_string(), 0.3);
        explanation.feature_attributions.insert("feature_c".to_string(), 0.7);

        let top = explanation.top_features(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "feature_a");
        assert_eq!(top[1].0, "feature_c");
    }
}
