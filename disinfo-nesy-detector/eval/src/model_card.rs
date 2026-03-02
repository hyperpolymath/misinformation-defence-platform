// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Model Card generation for ML transparency
//!
//! Implements the Model Card framework for documenting ML models:
//! - Mitchell et al. (2019) "Model Cards for Model Reporting"
//! - Follows Hugging Face and Google model card conventions
//!
//! Model cards provide structured documentation about:
//! - Model details and intended use
//! - Training data and methodology
//! - Evaluation results and limitations
//! - Ethical considerations and bias analysis

use crate::explainability::ExplainabilityMetrics;
use crate::metrics::EvaluationMetrics;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete model card following industry standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCard {
    /// Model identification and metadata
    pub model_details: ModelDetails,
    /// Intended use cases and users
    pub intended_use: IntendedUse,
    /// Factors affecting model behavior
    pub factors: Factors,
    /// Evaluation metrics and results
    pub metrics: MetricsSection,
    /// Training and evaluation data
    pub data: DataSection,
    /// Ethical considerations
    pub ethical_considerations: EthicalConsiderations,
    /// Caveats and recommendations
    pub caveats_and_recommendations: CaveatsAndRecommendations,
    /// Quantitative analyses
    pub quantitative_analyses: QuantitativeAnalyses,
    /// Model card metadata
    pub card_metadata: CardMetadata,
}

/// Model identification and basic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetails {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model type/architecture
    pub model_type: String,
    /// Brief description
    pub description: String,
    /// Authors/developers
    pub developers: Vec<String>,
    /// Organization
    pub organization: String,
    /// License
    pub license: String,
    /// Model date
    pub date: DateTime<Utc>,
    /// Citation information
    pub citation: Option<String>,
    /// Contact information
    pub contact: Option<String>,
    /// Link to model weights/code
    pub model_uri: Option<String>,
    /// Framework used (e.g., "Rust + ONNX")
    pub framework: String,
    /// Additional model information
    pub additional_info: HashMap<String, String>,
}

impl Default for ModelDetails {
    fn default() -> Self {
        Self {
            name: "Unnamed Model".to_string(),
            version: "0.1.0".to_string(),
            model_type: "Unknown".to_string(),
            description: String::new(),
            developers: Vec::new(),
            organization: String::new(),
            license: "AGPL-3.0-or-later".to_string(),
            date: Utc::now(),
            citation: None,
            contact: None,
            model_uri: None,
            framework: "Rust".to_string(),
            additional_info: HashMap::new(),
        }
    }
}

/// Intended use cases and users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntendedUse {
    /// Primary intended uses
    pub primary_uses: Vec<String>,
    /// Primary intended users
    pub primary_users: Vec<String>,
    /// Out-of-scope uses (what the model should NOT be used for)
    pub out_of_scope_uses: Vec<String>,
}

impl Default for IntendedUse {
    fn default() -> Self {
        Self {
            primary_uses: vec![
                "Detecting potential disinformation in text content".to_string(),
                "Flagging suspicious content for human review".to_string(),
                "Research on disinformation detection methods".to_string(),
            ],
            primary_users: vec![
                "Fact-checking organizations".to_string(),
                "Researchers studying disinformation".to_string(),
                "Content moderation teams (with human oversight)".to_string(),
            ],
            out_of_scope_uses: vec![
                "Automated content removal without human review".to_string(),
                "Determining legal liability for content".to_string(),
                "Surveillance or targeting of individuals".to_string(),
                "Censorship of legitimate political discourse".to_string(),
            ],
        }
    }
}

/// Factors that affect model performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factors {
    /// Relevant demographic groups
    pub groups: Vec<FactorGroup>,
    /// Instrumentation details
    pub instrumentation: Vec<String>,
    /// Environmental factors
    pub environment: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorGroup {
    /// Group name
    pub name: String,
    /// Description of the group
    pub description: String,
    /// Expected performance variation
    pub performance_notes: String,
}

impl Default for Factors {
    fn default() -> Self {
        Self {
            groups: vec![
                FactorGroup {
                    name: "Political orientation".to_string(),
                    description: "Content from different political perspectives".to_string(),
                    performance_notes: "Model should be evaluated for balanced performance across political spectrum".to_string(),
                },
                FactorGroup {
                    name: "Language/dialect".to_string(),
                    description: "English language variations".to_string(),
                    performance_notes: "Currently optimized for US English; may vary for other dialects".to_string(),
                },
                FactorGroup {
                    name: "Content domain".to_string(),
                    description: "Topic areas (politics, health, science, etc.)".to_string(),
                    performance_notes: "Performance may vary by domain; training data composition affects this".to_string(),
                },
            ],
            instrumentation: vec![
                "Text input only (no multimodal analysis in current version)".to_string(),
            ],
            environment: vec![
                "Designed for online content moderation contexts".to_string(),
                "Requires human oversight for final decisions".to_string(),
            ],
        }
    }
}

/// Metrics and evaluation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSection {
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Explainability metrics
    pub explainability: Option<ExplainabilityMetrics>,
    /// Decision thresholds
    pub thresholds: Vec<ThresholdInfo>,
    /// Disaggregated metrics by subgroup
    pub disaggregated: HashMap<String, PerformanceMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub mcc: f64,
    pub auc_roc: Option<f64>,
    pub calibration_error: Option<f64>,
}

impl From<&EvaluationMetrics> for PerformanceMetrics {
    fn from(metrics: &EvaluationMetrics) -> Self {
        Self {
            accuracy: metrics.classification.accuracy,
            precision: metrics.classification.precision,
            recall: metrics.classification.recall,
            f1_score: metrics.classification.f1_score,
            mcc: metrics.classification.mcc,
            auc_roc: metrics.auc_roc,
            calibration_error: metrics.brier_score,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdInfo {
    /// Threshold name
    pub name: String,
    /// Threshold value
    pub value: f64,
    /// Description
    pub description: String,
}

impl Default for MetricsSection {
    fn default() -> Self {
        Self {
            performance: PerformanceMetrics {
                accuracy: 0.0,
                precision: 0.0,
                recall: 0.0,
                f1_score: 0.0,
                mcc: 0.0,
                auc_roc: None,
                calibration_error: None,
            },
            explainability: None,
            thresholds: vec![
                ThresholdInfo {
                    name: "High confidence".to_string(),
                    value: 0.9,
                    description: "Predictions above this threshold are high confidence".to_string(),
                },
                ThresholdInfo {
                    name: "Decision boundary".to_string(),
                    value: 0.5,
                    description: "Default classification threshold".to_string(),
                },
            ],
            disaggregated: HashMap::new(),
        }
    }
}

/// Training and evaluation data information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSection {
    /// Training data description
    pub training_data: DataDescription,
    /// Evaluation data description
    pub evaluation_data: DataDescription,
    /// Data preprocessing steps
    pub preprocessing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDescription {
    /// Dataset name(s)
    pub datasets: Vec<String>,
    /// Total size
    pub size: Option<usize>,
    /// Label distribution
    pub label_distribution: HashMap<String, f64>,
    /// Time period covered
    pub time_period: Option<String>,
    /// Geographic coverage
    pub geographic_coverage: Option<String>,
    /// Data collection methodology
    pub methodology: String,
    /// Known limitations
    pub limitations: Vec<String>,
}

impl Default for DataDescription {
    fn default() -> Self {
        Self {
            datasets: Vec::new(),
            size: None,
            label_distribution: HashMap::new(),
            time_period: None,
            geographic_coverage: None,
            methodology: String::new(),
            limitations: Vec::new(),
        }
    }
}

impl Default for DataSection {
    fn default() -> Self {
        Self {
            training_data: DataDescription::default(),
            evaluation_data: DataDescription::default(),
            preprocessing: vec![
                "Text normalization (lowercase, unicode normalization)".to_string(),
                "Removal of URLs and special characters".to_string(),
                "Tokenization".to_string(),
            ],
        }
    }
}

/// Ethical considerations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConsiderations {
    /// Potential harms and mitigations
    pub potential_harms: Vec<HarmConsideration>,
    /// Fairness considerations
    pub fairness: FairnessConsiderations,
    /// Privacy considerations
    pub privacy: Vec<String>,
    /// Human oversight requirements
    pub human_oversight: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmConsideration {
    /// Type of potential harm
    pub harm_type: String,
    /// Description
    pub description: String,
    /// Mitigation strategies
    pub mitigations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairnessConsiderations {
    /// Fairness definition used
    pub definition: String,
    /// Groups considered
    pub groups_evaluated: Vec<String>,
    /// Known disparities
    pub known_disparities: Vec<String>,
    /// Mitigation efforts
    pub mitigation_efforts: Vec<String>,
}

impl Default for EthicalConsiderations {
    fn default() -> Self {
        Self {
            potential_harms: vec![
                HarmConsideration {
                    harm_type: "False positives".to_string(),
                    description: "Legitimate content incorrectly flagged as disinformation".to_string(),
                    mitigations: vec![
                        "Human review before any action is taken".to_string(),
                        "Appeal mechanisms for content creators".to_string(),
                        "Transparency about flagging decisions".to_string(),
                    ],
                },
                HarmConsideration {
                    harm_type: "False negatives".to_string(),
                    description: "Disinformation not detected by the model".to_string(),
                    mitigations: vec![
                        "Regular model updates with new disinformation patterns".to_string(),
                        "Complementary human fact-checking".to_string(),
                        "Multiple detection methods in parallel".to_string(),
                    ],
                },
                HarmConsideration {
                    harm_type: "Political bias".to_string(),
                    description: "Model may exhibit political bias in classifications".to_string(),
                    mitigations: vec![
                        "Regular bias audits across political spectrum".to_string(),
                        "Balanced training data curation".to_string(),
                        "Transparent evaluation methodology".to_string(),
                    ],
                },
            ],
            fairness: FairnessConsiderations {
                definition: "Equalized odds across demographic and political groups".to_string(),
                groups_evaluated: vec![
                    "Political orientation (liberal, conservative, centrist)".to_string(),
                    "Content domain (politics, health, science, entertainment)".to_string(),
                ],
                known_disparities: Vec::new(),
                mitigation_efforts: vec![
                    "Stratified evaluation across groups".to_string(),
                    "Regular fairness audits".to_string(),
                ],
            },
            privacy: vec![
                "Model does not store or log analyzed content".to_string(),
                "No personally identifiable information in training data".to_string(),
                "Inference can be run locally without data transmission".to_string(),
            ],
            human_oversight: vec![
                "All flagged content should be reviewed by trained human moderators".to_string(),
                "Model predictions are recommendations, not final decisions".to_string(),
                "Users should be informed when AI-assisted moderation is used".to_string(),
            ],
        }
    }
}

/// Caveats, limitations, and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaveatsAndRecommendations {
    /// Known caveats and limitations
    pub caveats: Vec<String>,
    /// Recommendations for use
    pub recommendations: Vec<String>,
    /// Known failure modes
    pub failure_modes: Vec<String>,
}

impl Default for CaveatsAndRecommendations {
    fn default() -> Self {
        Self {
            caveats: vec![
                "Model is trained on English text only".to_string(),
                "Performance may degrade on content types not in training data".to_string(),
                "Adversarial content specifically designed to evade detection may succeed".to_string(),
                "Model reflects patterns in training data which may have its own biases".to_string(),
            ],
            recommendations: vec![
                "Use as one signal among many in content moderation workflow".to_string(),
                "Maintain human oversight for all consequential decisions".to_string(),
                "Regularly evaluate model on new data to detect drift".to_string(),
                "Provide transparency to users about AI-assisted moderation".to_string(),
                "Establish clear appeals process for flagged content".to_string(),
            ],
            failure_modes: vec![
                "Satire and parody may be misclassified".to_string(),
                "Emerging disinformation narratives not in training data".to_string(),
                "Content mixing true and false information".to_string(),
                "Sophisticated state-sponsored disinformation campaigns".to_string(),
            ],
        }
    }
}

/// Quantitative analyses (disaggregated evaluations, intersectional analysis)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantitativeAnalyses {
    /// Unitary analyses (single-factor evaluations)
    pub unitary: HashMap<String, PerformanceMetrics>,
    /// Intersectional analyses (multi-factor evaluations)
    pub intersectional: HashMap<String, PerformanceMetrics>,
}

impl Default for QuantitativeAnalyses {
    fn default() -> Self {
        Self {
            unitary: HashMap::new(),
            intersectional: HashMap::new(),
        }
    }
}

/// Model card metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMetadata {
    /// Schema version
    pub schema_version: String,
    /// Card creation date
    pub created: DateTime<Utc>,
    /// Last updated date
    pub last_updated: DateTime<Utc>,
    /// Card authors
    pub authors: Vec<String>,
}

impl Default for CardMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            schema_version: "1.0.0".to_string(),
            created: now,
            last_updated: now,
            authors: Vec::new(),
        }
    }
}

impl ModelCard {
    /// Create a new model card with defaults
    pub fn new(name: &str, version: &str, model_type: &str) -> Self {
        Self {
            model_details: ModelDetails {
                name: name.to_string(),
                version: version.to_string(),
                model_type: model_type.to_string(),
                ..Default::default()
            },
            intended_use: IntendedUse::default(),
            factors: Factors::default(),
            metrics: MetricsSection::default(),
            data: DataSection::default(),
            ethical_considerations: EthicalConsiderations::default(),
            caveats_and_recommendations: CaveatsAndRecommendations::default(),
            quantitative_analyses: QuantitativeAnalyses::default(),
            card_metadata: CardMetadata::default(),
        }
    }

    /// Set performance metrics from evaluation results
    pub fn set_metrics(&mut self, metrics: &EvaluationMetrics) {
        self.metrics.performance = PerformanceMetrics::from(metrics);
    }

    /// Set explainability metrics
    pub fn set_explainability_metrics(&mut self, metrics: ExplainabilityMetrics) {
        self.metrics.explainability = Some(metrics);
    }

    /// Add a disaggregated metric result
    pub fn add_disaggregated_metric(&mut self, group: &str, metrics: PerformanceMetrics) {
        self.metrics.disaggregated.insert(group.to_string(), metrics);
    }

    /// Generate markdown representation
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        // Title
        md.push_str(&format!("# Model Card: {}\n\n", self.model_details.name));

        // Model Details
        md.push_str("## Model Details\n\n");
        md.push_str(&format!("- **Version:** {}\n", self.model_details.version));
        md.push_str(&format!("- **Type:** {}\n", self.model_details.model_type));
        md.push_str(&format!("- **Framework:** {}\n", self.model_details.framework));
        md.push_str(&format!("- **License:** {}\n", self.model_details.license));
        md.push_str(&format!("- **Date:** {}\n", self.model_details.date.format("%Y-%m-%d")));
        if !self.model_details.developers.is_empty() {
            md.push_str(&format!("- **Developers:** {}\n", self.model_details.developers.join(", ")));
        }
        if !self.model_details.description.is_empty() {
            md.push_str(&format!("\n{}\n", self.model_details.description));
        }
        md.push('\n');

        // Intended Use
        md.push_str("## Intended Use\n\n");
        md.push_str("### Primary Uses\n\n");
        for use_case in &self.intended_use.primary_uses {
            md.push_str(&format!("- {}\n", use_case));
        }
        md.push_str("\n### Primary Users\n\n");
        for user in &self.intended_use.primary_users {
            md.push_str(&format!("- {}\n", user));
        }
        md.push_str("\n### Out-of-Scope Uses\n\n");
        md.push_str("⚠️ **The following uses are explicitly out of scope:**\n\n");
        for oos in &self.intended_use.out_of_scope_uses {
            md.push_str(&format!("- {}\n", oos));
        }
        md.push('\n');

        // Metrics
        md.push_str("## Performance Metrics\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Accuracy | {:.4} |\n", self.metrics.performance.accuracy));
        md.push_str(&format!("| Precision | {:.4} |\n", self.metrics.performance.precision));
        md.push_str(&format!("| Recall | {:.4} |\n", self.metrics.performance.recall));
        md.push_str(&format!("| F1 Score | {:.4} |\n", self.metrics.performance.f1_score));
        md.push_str(&format!("| MCC | {:.4} |\n", self.metrics.performance.mcc));
        if let Some(auc) = self.metrics.performance.auc_roc {
            md.push_str(&format!("| AUC-ROC | {:.4} |\n", auc));
        }
        md.push('\n');

        // Explainability Metrics
        if let Some(ref exp_metrics) = self.metrics.explainability {
            md.push_str("### Explainability Metrics\n\n");
            md.push_str("| Metric | Value |\n");
            md.push_str("|--------|-------|\n");
            md.push_str(&format!("| Avg Evidence Count | {:.2} |\n", exp_metrics.avg_evidence_count));
            md.push_str(&format!("| Completeness Score | {:.1}% |\n", exp_metrics.avg_completeness * 100.0));
            md.push_str(&format!("| Feature Consistency | {:.1}% |\n", exp_metrics.feature_consistency * 100.0));
            md.push_str(&format!("| Calibration Error | {:.4} |\n", exp_metrics.calibration_error));
            md.push('\n');
        }

        // Disaggregated Metrics
        if !self.metrics.disaggregated.is_empty() {
            md.push_str("### Disaggregated Performance\n\n");
            md.push_str("| Group | Accuracy | F1 | MCC |\n");
            md.push_str("|-------|----------|-----|-----|\n");
            for (group, metrics) in &self.metrics.disaggregated {
                md.push_str(&format!(
                    "| {} | {:.4} | {:.4} | {:.4} |\n",
                    group, metrics.accuracy, metrics.f1_score, metrics.mcc
                ));
            }
            md.push('\n');
        }

        // Data
        md.push_str("## Training Data\n\n");
        if !self.data.training_data.datasets.is_empty() {
            md.push_str("**Datasets:**\n");
            for ds in &self.data.training_data.datasets {
                md.push_str(&format!("- {}\n", ds));
            }
        }
        if let Some(size) = self.data.training_data.size {
            md.push_str(&format!("\n**Size:** {} samples\n", size));
        }
        if !self.data.training_data.methodology.is_empty() {
            md.push_str(&format!("\n**Methodology:** {}\n", self.data.training_data.methodology));
        }
        md.push('\n');

        // Ethical Considerations
        md.push_str("## Ethical Considerations\n\n");
        md.push_str("### Potential Harms and Mitigations\n\n");
        for harm in &self.ethical_considerations.potential_harms {
            md.push_str(&format!("#### {}\n\n", harm.harm_type));
            md.push_str(&format!("{}\n\n", harm.description));
            md.push_str("**Mitigations:**\n");
            for m in &harm.mitigations {
                md.push_str(&format!("- {}\n", m));
            }
            md.push('\n');
        }

        md.push_str("### Human Oversight Requirements\n\n");
        for req in &self.ethical_considerations.human_oversight {
            md.push_str(&format!("- {}\n", req));
        }
        md.push('\n');

        // Caveats and Recommendations
        md.push_str("## Caveats and Recommendations\n\n");
        md.push_str("### Known Limitations\n\n");
        for caveat in &self.caveats_and_recommendations.caveats {
            md.push_str(&format!("- {}\n", caveat));
        }
        md.push_str("\n### Recommendations\n\n");
        for rec in &self.caveats_and_recommendations.recommendations {
            md.push_str(&format!("- {}\n", rec));
        }
        md.push_str("\n### Known Failure Modes\n\n");
        for fm in &self.caveats_and_recommendations.failure_modes {
            md.push_str(&format!("- {}\n", fm));
        }
        md.push('\n');

        // Card Metadata
        md.push_str("---\n\n");
        md.push_str(&format!(
            "*Model Card generated on {} (schema v{})*\n",
            self.card_metadata.last_updated.format("%Y-%m-%d"),
            self.card_metadata.schema_version
        ));

        md
    }

    /// Save model card to file
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let md = self.to_markdown();
        std::fs::write(path, md)?;
        Ok(())
    }

    /// Save as JSON
    pub fn save_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// Builder for creating model cards
pub struct ModelCardBuilder {
    card: ModelCard,
}

impl ModelCardBuilder {
    pub fn new(name: &str, version: &str, model_type: &str) -> Self {
        Self {
            card: ModelCard::new(name, version, model_type),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.card.model_details.description = desc.to_string();
        self
    }

    pub fn developers(mut self, devs: Vec<String>) -> Self {
        self.card.model_details.developers = devs;
        self
    }

    pub fn organization(mut self, org: &str) -> Self {
        self.card.model_details.organization = org.to_string();
        self
    }

    pub fn license(mut self, license: &str) -> Self {
        self.card.model_details.license = license.to_string();
        self
    }

    pub fn framework(mut self, framework: &str) -> Self {
        self.card.model_details.framework = framework.to_string();
        self
    }

    pub fn citation(mut self, citation: &str) -> Self {
        self.card.model_details.citation = Some(citation.to_string());
        self
    }

    pub fn metrics(mut self, metrics: &EvaluationMetrics) -> Self {
        self.card.set_metrics(metrics);
        self
    }

    pub fn explainability_metrics(mut self, metrics: ExplainabilityMetrics) -> Self {
        self.card.set_explainability_metrics(metrics);
        self
    }

    pub fn training_datasets(mut self, datasets: Vec<String>) -> Self {
        self.card.data.training_data.datasets = datasets;
        self
    }

    pub fn training_size(mut self, size: usize) -> Self {
        self.card.data.training_data.size = Some(size);
        self
    }

    pub fn add_caveat(mut self, caveat: &str) -> Self {
        self.card.caveats_and_recommendations.caveats.push(caveat.to_string());
        self
    }

    pub fn add_recommendation(mut self, rec: &str) -> Self {
        self.card.caveats_and_recommendations.recommendations.push(rec.to_string());
        self
    }

    pub fn build(self) -> ModelCard {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_card_builder() {
        let card = ModelCardBuilder::new("TestModel", "1.0.0", "TF-IDF Classifier")
            .description("A test model for disinformation detection")
            .developers(vec!["Test Author".to_string()])
            .organization("Test Org")
            .training_datasets(vec!["LIAR".to_string(), "ISOT".to_string()])
            .training_size(10000)
            .build();

        assert_eq!(card.model_details.name, "TestModel");
        assert_eq!(card.model_details.version, "1.0.0");
        assert_eq!(card.data.training_data.datasets.len(), 2);
    }

    #[test]
    fn test_model_card_markdown() {
        let card = ModelCard::new("TestModel", "1.0.0", "Baseline");
        let md = card.to_markdown();

        assert!(md.contains("# Model Card: TestModel"));
        assert!(md.contains("## Model Details"));
        assert!(md.contains("## Intended Use"));
        assert!(md.contains("## Ethical Considerations"));
    }

    #[test]
    fn test_performance_metrics_from_evaluation() {
        let metrics = crate::metrics::EvaluationMetrics::from_predictions(
            &[crate::datasets::Label::Disinformation, crate::datasets::Label::Authentic],
            &[crate::datasets::Label::Disinformation, crate::datasets::Label::Authentic],
        );

        let perf = PerformanceMetrics::from(&metrics);
        assert_eq!(perf.accuracy, 1.0);
        assert_eq!(perf.f1_score, 1.0);
    }
}
