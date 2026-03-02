// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Reproducible evaluation pipeline for disinformation detection
//!
//! Orchestrates:
//! - Dataset loading
//! - Baseline model training and evaluation
//! - Metrics computation
//! - Explainability analysis
//! - Model card generation
//! - Results serialization

use crate::baselines::{all_baselines, BaselineModel};
use crate::datasets::{Dataset, Label, Sample};
use crate::explainability::{Explanation, ExplainabilityMetrics};
use crate::metrics::EvaluationMetrics;
use crate::model_card::{ModelCard, ModelCardBuilder};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for the evaluation pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfig {
    /// Random seed for reproducibility
    pub seed: u64,
    /// Dataset to evaluate on
    pub dataset_id: String,
    /// Path to dataset directory (if loading from disk)
    pub dataset_path: Option<String>,
    /// Which split to evaluate on ("test", "validation", "train")
    pub eval_split: String,
    /// Whether to run all baselines
    pub run_baselines: bool,
    /// Specific baselines to run (empty = all)
    pub baseline_names: Vec<String>,
    /// Output directory for results
    pub output_dir: String,
}

impl Default for EvaluationConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            dataset_id: "synthetic".to_string(),
            dataset_path: None,
            eval_split: "test".to_string(),
            run_baselines: true,
            baseline_names: vec![],
            output_dir: "eval/results".to_string(),
        }
    }
}

/// Results from a single model evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResult {
    pub model_name: String,
    pub model_description: String,
    pub metrics: EvaluationMetrics,
    pub explainability_metrics: Option<ExplainabilityMetrics>,
    pub predictions_sample: Vec<PredictionSample>,
    pub explanations_sample: Vec<ExplanationSample>,
    pub training_samples: usize,
    pub eval_samples: usize,
    pub supports_explanations: bool,
}

/// A sample prediction for inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionSample {
    pub id: String,
    pub text_preview: String,
    pub predicted: String,
    pub actual: String,
    pub probability: f64,
    pub correct: bool,
}

/// A sample explanation for inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationSample {
    pub id: String,
    pub text_preview: String,
    pub prediction: String,
    pub confidence: f64,
    pub summary: String,
    pub evidence_count: usize,
    pub top_features: Vec<(String, f64)>,
}

/// Complete evaluation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResults {
    pub config: EvaluationConfig,
    pub dataset_info: DatasetInfo,
    pub baseline_results: Vec<ModelResult>,
    pub summary: EvaluationSummary,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub id: String,
    pub name: String,
    pub total_samples: usize,
    pub train_samples: usize,
    pub validation_samples: usize,
    pub test_samples: usize,
    pub label_distribution: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationSummary {
    pub best_model: String,
    pub best_f1: f64,
    pub best_accuracy: f64,
    pub baseline_comparison: Vec<BaselineComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineComparison {
    pub model: String,
    pub accuracy: f64,
    pub f1_score: f64,
    pub mcc: f64,
    pub auc_roc: Option<f64>,
}

/// Main evaluation pipeline
pub struct EvaluationPipeline {
    config: EvaluationConfig,
    dataset: Option<Dataset>,
}

impl EvaluationPipeline {
    pub fn new(config: EvaluationConfig) -> Self {
        Self {
            config,
            dataset: None,
        }
    }

    /// Load dataset based on configuration
    pub fn load_dataset(&mut self) -> Result<()> {
        let dataset = if self.config.dataset_id == "synthetic" {
            tracing::info!("Loading synthetic dataset with seed {}", self.config.seed);
            Dataset::load_synthetic(1000, self.config.seed)
        } else if let Some(ref path) = self.config.dataset_path {
            let path = Path::new(path);
            match self.config.dataset_id.as_str() {
                "liar" => {
                    tracing::info!("Loading LIAR dataset from {}", path.display());
                    Dataset::load_liar(path)?
                }
                "isot" => {
                    tracing::info!("Loading ISOT dataset from {}", path.display());
                    Dataset::load_isot(path)?
                }
                _ => {
                    tracing::warn!("Unknown dataset '{}', falling back to synthetic", self.config.dataset_id);
                    Dataset::load_synthetic(1000, self.config.seed)
                }
            }
        } else {
            tracing::warn!("No dataset path provided, using synthetic dataset");
            Dataset::load_synthetic(1000, self.config.seed)
        };

        tracing::info!(
            "Dataset loaded: {} samples (train={}, val={}, test={})",
            dataset.total_samples(),
            dataset.train.len(),
            dataset.validation.len(),
            dataset.test.len()
        );

        self.dataset = Some(dataset);
        Ok(())
    }

    /// Get the evaluation split samples
    fn get_eval_samples(&self) -> &[Sample] {
        let dataset = self.dataset.as_ref().expect("Dataset not loaded");
        match self.config.eval_split.as_str() {
            "train" => &dataset.train,
            "validation" | "val" => &dataset.validation,
            "test" | _ => &dataset.test,
        }
    }

    /// Get training samples
    fn get_train_samples(&self) -> &[Sample] {
        let dataset = self.dataset.as_ref().expect("Dataset not loaded");
        &dataset.train
    }

    /// Evaluate a single model
    fn evaluate_model(&self, model: &dyn BaselineModel, eval_samples: &[Sample]) -> ModelResult {
        let supports_explanations = model.supports_explanations();

        // Get predictions with explanations if supported
        let predictions_with_explanations = model.predict_batch_with_explanations(eval_samples);

        let pred_labels: Vec<Label> = predictions_with_explanations.iter().map(|p| p.prediction.label).collect();
        let true_labels: Vec<Label> = eval_samples.iter().map(|s| s.label).collect();
        let probabilities: Vec<f64> = predictions_with_explanations.iter().map(|p| p.prediction.probability).collect();

        let metrics = EvaluationMetrics::from_predictions_with_probs(&pred_labels, &true_labels, &probabilities);

        // Compute explainability metrics if model supports explanations
        let explanations: Vec<Explanation> = predictions_with_explanations.iter().map(|p| p.explanation.clone()).collect();
        let explainability_metrics = if supports_explanations {
            Some(ExplainabilityMetrics::from_explanations(&explanations, &true_labels, &probabilities))
        } else {
            None
        };

        // Sample predictions for inspection (first 10 errors, first 10 correct)
        let mut prediction_samples = Vec::new();
        let mut errors = 0;
        let mut corrects = 0;

        for (pred_exp, sample) in predictions_with_explanations.iter().zip(eval_samples.iter()) {
            let correct = pred_exp.prediction.label == sample.label;
            if (!correct && errors < 10) || (correct && corrects < 10) {
                prediction_samples.push(PredictionSample {
                    id: sample.id.clone(),
                    text_preview: sample.text.chars().take(100).collect::<String>() + "...",
                    predicted: format!("{:?}", pred_exp.prediction.label),
                    actual: format!("{:?}", sample.label),
                    probability: pred_exp.prediction.probability,
                    correct,
                });
                if correct {
                    corrects += 1;
                } else {
                    errors += 1;
                }
            }
            if errors >= 10 && corrects >= 10 {
                break;
            }
        }

        // Sample explanations (first 5 with non-empty evidence)
        let mut explanations_sample = Vec::new();
        for (pred_exp, sample) in predictions_with_explanations.iter().zip(eval_samples.iter()) {
            if explanations_sample.len() >= 5 {
                break;
            }
            if !pred_exp.explanation.evidence.is_empty() || !pred_exp.explanation.reasoning_trace.is_empty() {
                let top_features: Vec<(String, f64)> = pred_exp.explanation.top_features(3)
                    .into_iter()
                    .map(|(k, v)| (k.clone(), *v))
                    .collect();

                explanations_sample.push(ExplanationSample {
                    id: sample.id.clone(),
                    text_preview: sample.text.chars().take(80).collect::<String>() + "...",
                    prediction: format!("{:?}", pred_exp.prediction.label),
                    confidence: pred_exp.prediction.probability,
                    summary: pred_exp.explanation.summary.clone(),
                    evidence_count: pred_exp.explanation.evidence.len(),
                    top_features,
                });
            }
        }

        ModelResult {
            model_name: model.name().to_string(),
            model_description: model.description().to_string(),
            metrics,
            explainability_metrics,
            predictions_sample: prediction_samples,
            explanations_sample,
            training_samples: self.get_train_samples().len(),
            eval_samples: eval_samples.len(),
            supports_explanations,
        }
    }

    /// Generate a model card for a specific model result
    pub fn generate_model_card(result: &ModelResult, dataset_info: &DatasetInfo) -> ModelCard {
        let mut builder = ModelCardBuilder::new(
            &result.model_name,
            env!("CARGO_PKG_VERSION"),
            &result.model_description,
        )
        .description(&format!(
            "Baseline model for disinformation detection. {}",
            result.model_description
        ))
        .organization("Hyperpolymath")
        .license("AGPL-3.0-or-later")
        .framework("Rust")
        .metrics(&result.metrics)
        .training_datasets(vec![dataset_info.name.clone()])
        .training_size(result.training_samples);

        // Add explainability metrics if available
        if let Some(ref exp_metrics) = result.explainability_metrics {
            builder = builder.explainability_metrics(exp_metrics.clone());
        }

        // Add model-specific caveats
        if !result.supports_explanations {
            builder = builder.add_caveat("This model does not provide detailed explanations for its predictions");
        }

        builder.build()
    }

    /// Run the full evaluation pipeline
    pub fn run(&mut self) -> Result<EvaluationResults> {
        if self.dataset.is_none() {
            self.load_dataset()?;
        }

        let dataset = self.dataset.as_ref().unwrap();
        let train_samples = self.get_train_samples();
        let eval_samples = self.get_eval_samples();

        // Create dataset info
        let label_dist = Dataset::label_distribution(eval_samples);
        let dataset_info = DatasetInfo {
            id: dataset.config.id.clone(),
            name: dataset.config.name.clone(),
            total_samples: dataset.total_samples(),
            train_samples: dataset.train.len(),
            validation_samples: dataset.validation.len(),
            test_samples: dataset.test.len(),
            label_distribution: label_dist
                .iter()
                .map(|(k, v)| (format!("{:?}", k), *v))
                .collect(),
        };

        // Run baseline evaluations
        let mut baseline_results = Vec::new();
        let baselines = all_baselines(self.config.seed);

        for mut baseline in baselines {
            let name = baseline.name().to_string();

            // Filter if specific baselines requested
            if !self.config.baseline_names.is_empty()
                && !self.config.baseline_names.contains(&name)
            {
                continue;
            }

            tracing::info!("Evaluating baseline: {}", name);

            // Train on training split
            baseline.train(train_samples);

            // Evaluate on eval split
            let result = self.evaluate_model(baseline.as_ref(), eval_samples);

            tracing::info!(
                "  {} - Accuracy: {:.4}, F1: {:.4}, MCC: {:.4}",
                result.model_name,
                result.metrics.classification.accuracy,
                result.metrics.classification.f1_score,
                result.metrics.classification.mcc
            );

            baseline_results.push(result);
        }

        // Generate summary
        let mut best_model = "None".to_string();
        let mut best_f1 = 0.0;
        let mut best_accuracy = 0.0;

        let baseline_comparison: Vec<_> = baseline_results
            .iter()
            .map(|r| {
                if r.metrics.classification.f1_score > best_f1 {
                    best_f1 = r.metrics.classification.f1_score;
                    best_accuracy = r.metrics.classification.accuracy;
                    best_model = r.model_name.clone();
                }
                BaselineComparison {
                    model: r.model_name.clone(),
                    accuracy: r.metrics.classification.accuracy,
                    f1_score: r.metrics.classification.f1_score,
                    mcc: r.metrics.classification.mcc,
                    auc_roc: r.metrics.auc_roc,
                }
            })
            .collect();

        let summary = EvaluationSummary {
            best_model,
            best_f1,
            best_accuracy,
            baseline_comparison,
        };

        Ok(EvaluationResults {
            config: self.config.clone(),
            dataset_info,
            baseline_results,
            summary,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    /// Save results to JSON file
    pub fn save_results(results: &EvaluationResults, output_path: &Path) -> Result<()> {
        std::fs::create_dir_all(output_path.parent().unwrap_or(Path::new(".")))?;
        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(output_path, json)?;
        tracing::info!("Results saved to {}", output_path.display());
        Ok(())
    }

    /// Generate a markdown report
    pub fn generate_report(results: &EvaluationResults) -> String {
        let mut report = String::new();

        report.push_str("# Disinformation Detection Evaluation Report\n\n");
        report.push_str(&format!("**Generated:** {}\n\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Version:** {}\n\n", results.version));

        report.push_str("## Dataset\n\n");
        report.push_str(&format!("- **ID:** {}\n", results.dataset_info.id));
        report.push_str(&format!("- **Name:** {}\n", results.dataset_info.name));
        report.push_str(&format!("- **Total Samples:** {}\n", results.dataset_info.total_samples));
        report.push_str(&format!(
            "- **Split Sizes:** Train={}, Val={}, Test={}\n",
            results.dataset_info.train_samples,
            results.dataset_info.validation_samples,
            results.dataset_info.test_samples
        ));
        report.push_str(&format!("- **Eval Split:** {}\n\n", results.config.eval_split));

        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Best Model:** {} (F1={:.4}, Accuracy={:.4})\n\n",
            results.summary.best_model, results.summary.best_f1, results.summary.best_accuracy));

        report.push_str("### Baseline Comparison\n\n");
        report.push_str("| Model | Accuracy | F1 Score | MCC | AUC-ROC | Explainable |\n");
        report.push_str("|-------|----------|----------|-----|---------|-------------|\n");

        for result in &results.baseline_results {
            let auc = result.metrics.auc_roc.map_or("-".to_string(), |v| format!("{:.4}", v));
            let explainable = if result.supports_explanations { "✓" } else { "-" };
            report.push_str(&format!(
                "| {} | {:.4} | {:.4} | {:.4} | {} | {} |\n",
                result.model_name,
                result.metrics.classification.accuracy,
                result.metrics.classification.f1_score,
                result.metrics.classification.mcc,
                auc,
                explainable
            ));
        }

        report.push_str("\n## Detailed Results\n\n");

        for result in &results.baseline_results {
            report.push_str(&format!("### {}\n\n", result.model_name));
            report.push_str(&format!("*{}*\n\n", result.model_description));
            report.push_str(&format!("- Training samples: {}\n", result.training_samples));
            report.push_str(&format!("- Evaluation samples: {}\n", result.eval_samples));
            report.push_str(&format!("- Supports explanations: {}\n\n", result.supports_explanations));

            report.push_str("#### Performance Metrics\n\n");
            report.push_str(&format!("```\n{}\n```\n\n", result.metrics.classification.format()));

            // Add explainability metrics if available
            if let Some(ref exp_metrics) = result.explainability_metrics {
                report.push_str("#### Explainability Metrics\n\n");
                report.push_str(&format!("```\n{}\n```\n\n", exp_metrics.format()));
            }

            // Add sample explanations if available
            if !result.explanations_sample.is_empty() {
                report.push_str("#### Sample Explanations\n\n");
                for (i, exp) in result.explanations_sample.iter().take(3).enumerate() {
                    report.push_str(&format!("**Example {}:** {}\n\n", i + 1, exp.text_preview));
                    report.push_str(&format!("- Prediction: {} (confidence: {:.2}%)\n", exp.prediction, exp.confidence * 100.0));
                    report.push_str(&format!("- Evidence count: {}\n", exp.evidence_count));
                    if !exp.top_features.is_empty() {
                        report.push_str("- Top features:\n");
                        for (feat, score) in &exp.top_features {
                            report.push_str(&format!("  - {}: {:.4}\n", feat, score));
                        }
                    }
                    report.push('\n');
                }
            }
        }

        report.push_str("## Configuration\n\n");
        report.push_str(&format!("```json\n{}\n```\n", serde_json::to_string_pretty(&results.config).unwrap_or_default()));

        report
    }

    /// Generate and save model cards for all evaluated models
    pub fn save_model_cards(results: &EvaluationResults, output_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
        let cards_dir = output_dir.join("model_cards");
        std::fs::create_dir_all(&cards_dir)?;

        let mut saved_paths = Vec::new();

        for result in &results.baseline_results {
            let card = Self::generate_model_card(result, &results.dataset_info);

            // Save as markdown
            let md_filename = format!("{}_model_card.md", result.model_name.to_lowercase().replace(' ', "_"));
            let md_path = cards_dir.join(&md_filename);
            card.save(&md_path)?;
            saved_paths.push(md_path.clone());

            // Save as JSON
            let json_filename = format!("{}_model_card.json", result.model_name.to_lowercase().replace(' ', "_"));
            let json_path = cards_dir.join(&json_filename);
            card.save_json(&json_path)?;
            saved_paths.push(json_path);

            tracing::info!("Model card saved: {}", md_filename);
        }

        Ok(saved_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_synthetic() {
        let config = EvaluationConfig {
            seed: 42,
            dataset_id: "synthetic".to_string(),
            dataset_path: None,
            eval_split: "test".to_string(),
            run_baselines: true,
            baseline_names: vec![],
            output_dir: "test_output".to_string(),
        };

        let mut pipeline = EvaluationPipeline::new(config);
        let results = pipeline.run().expect("Pipeline should succeed");

        assert!(!results.baseline_results.is_empty());
        assert!(results.summary.best_f1 >= 0.0);
        assert!(results.summary.best_f1 <= 1.0);
    }

    #[test]
    fn test_pipeline_specific_baselines() {
        let config = EvaluationConfig {
            seed: 42,
            dataset_id: "synthetic".to_string(),
            dataset_path: None,
            eval_split: "test".to_string(),
            run_baselines: true,
            baseline_names: vec!["Random".to_string(), "Majority".to_string()],
            output_dir: "test_output".to_string(),
        };

        let mut pipeline = EvaluationPipeline::new(config);
        let results = pipeline.run().expect("Pipeline should succeed");

        assert_eq!(results.baseline_results.len(), 2);
    }

    #[test]
    fn test_generate_report() {
        let config = EvaluationConfig::default();
        let mut pipeline = EvaluationPipeline::new(config);
        let results = pipeline.run().expect("Pipeline should succeed");

        let report = EvaluationPipeline::generate_report(&results);

        assert!(report.contains("Disinformation Detection Evaluation Report"));
        assert!(report.contains("Baseline Comparison"));
        assert!(report.contains("Best Model"));
    }
}
