// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Evaluation pipeline for Neuro-Symbolic Disinformation Detector
//!
//! This crate provides:
//! - Dataset loading and preprocessing (LIAR, ISOT, FEVER, etc.)
//! - Evaluation metrics (Accuracy, Precision, Recall, F1, AUC-ROC)
//! - Baseline models (Random, Majority, TF-IDF + Logistic Regression)
//! - Explainability structures and metrics
//! - Model cards for ML transparency
//! - Reproducible evaluation pipeline with seeded randomness

pub mod baselines;
pub mod datasets;
pub mod explainability;
pub mod metrics;
pub mod model_card;
pub mod pipeline;

pub use baselines::{BaselineModel, MajorityBaseline, RandomBaseline, TfIdfBaseline};
pub use datasets::{Dataset, DatasetConfig, Label, Sample};
pub use explainability::{Evidence, EvidenceType, Explanation, ExplainabilityMetrics, Explainable};
pub use metrics::{ClassificationReport, ConfusionMatrix, EvaluationMetrics};
pub use model_card::{ModelCard, ModelCardBuilder};
pub use pipeline::{EvaluationConfig, EvaluationPipeline, EvaluationResults};
