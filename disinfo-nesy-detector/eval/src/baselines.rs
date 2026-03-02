// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Baseline models for disinformation detection evaluation
//!
//! Implements:
//! - Random baseline (uniform random predictions)
//! - Majority class baseline (always predict most common class)
//! - Stratified baseline (predict proportional to class distribution)
//! - TF-IDF + simple classifier baseline (keyword-based)
//!
//! All baselines support explainability through the `predict_with_explanation` method.

use crate::datasets::{Dataset, Label, Sample};
use crate::explainability::{Evidence, EvidenceType, Explanation, ExplanationBuilder};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Prediction output from a baseline model
#[derive(Debug, Clone)]
pub struct Prediction {
    pub label: Label,
    pub probability: f64, // P(disinformation)
}

/// Extended prediction with explanation
#[derive(Debug, Clone)]
pub struct PredictionWithExplanation {
    pub prediction: Prediction,
    pub explanation: Explanation,
}

/// Trait for all baseline models
pub trait BaselineModel: Send + Sync {
    /// Train the model on the given samples
    fn train(&mut self, samples: &[Sample]);

    /// Predict label for a single sample
    fn predict(&self, sample: &Sample) -> Prediction;

    /// Predict with explanation
    fn predict_with_explanation(&self, sample: &Sample) -> PredictionWithExplanation {
        let pred = self.predict(sample);
        let explanation = ExplanationBuilder::new(pred.label, pred.probability)
            .with_uncertainty("No detailed explanation available for this model type")
            .build();
        PredictionWithExplanation {
            prediction: pred,
            explanation,
        }
    }

    /// Predict labels for multiple samples
    fn predict_batch(&self, samples: &[Sample]) -> Vec<Prediction> {
        samples.iter().map(|s| self.predict(s)).collect()
    }

    /// Predict with explanations for multiple samples
    fn predict_batch_with_explanations(&self, samples: &[Sample]) -> Vec<PredictionWithExplanation> {
        samples.iter().map(|s| self.predict_with_explanation(s)).collect()
    }

    /// Get model name
    fn name(&self) -> &str;

    /// Get model description
    fn description(&self) -> &str;

    /// Check if this model provides meaningful explanations
    fn supports_explanations(&self) -> bool {
        false
    }
}

/// Random baseline: predicts uniformly at random
#[derive(Debug, Clone)]
pub struct RandomBaseline {
    seed: u64,
    rng: ChaCha8Rng,
}

impl RandomBaseline {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl BaselineModel for RandomBaseline {
    fn train(&mut self, _samples: &[Sample]) {
        // Reset RNG to ensure reproducibility
        self.rng = ChaCha8Rng::seed_from_u64(self.seed);
    }

    fn predict(&self, _sample: &Sample) -> Prediction {
        // Use interior mutability pattern for RNG
        let mut rng = self.rng.clone();
        let prob: f64 = rng.gen();
        Prediction {
            label: if prob > 0.5 { Label::Disinformation } else { Label::Authentic },
            probability: prob,
        }
    }

    fn predict_batch(&self, samples: &[Sample]) -> Vec<Prediction> {
        let mut rng = self.rng.clone();
        samples
            .iter()
            .map(|_| {
                let prob: f64 = rng.gen();
                Prediction {
                    label: if prob > 0.5 { Label::Disinformation } else { Label::Authentic },
                    probability: prob,
                }
            })
            .collect()
    }

    fn name(&self) -> &str {
        "Random"
    }

    fn description(&self) -> &str {
        "Uniform random predictions (expected accuracy: 50%)"
    }
}

/// Majority class baseline: always predicts the most common class
#[derive(Debug, Clone, Default)]
pub struct MajorityBaseline {
    majority_label: Option<Label>,
    class_prob: f64,
}

impl MajorityBaseline {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BaselineModel for MajorityBaseline {
    fn train(&mut self, samples: &[Sample]) {
        let dist = Dataset::label_distribution(samples);

        let disinfo_count = *dist.get(&Label::Disinformation).unwrap_or(&0);
        let authentic_count = *dist.get(&Label::Authentic).unwrap_or(&0);
        let total = disinfo_count + authentic_count;

        if disinfo_count > authentic_count {
            self.majority_label = Some(Label::Disinformation);
            self.class_prob = disinfo_count as f64 / total as f64;
        } else {
            self.majority_label = Some(Label::Authentic);
            self.class_prob = authentic_count as f64 / total as f64;
        }
    }

    fn predict(&self, _sample: &Sample) -> Prediction {
        match self.majority_label {
            Some(Label::Disinformation) => Prediction {
                label: Label::Disinformation,
                probability: self.class_prob,
            },
            _ => Prediction {
                label: Label::Authentic,
                probability: 1.0 - self.class_prob,
            },
        }
    }

    fn name(&self) -> &str {
        "Majority"
    }

    fn description(&self) -> &str {
        "Always predicts the majority class from training data"
    }
}

/// Stratified baseline: predicts proportionally to class distribution
#[derive(Debug, Clone)]
pub struct StratifiedBaseline {
    seed: u64,
    rng: ChaCha8Rng,
    disinfo_prob: f64,
}

impl StratifiedBaseline {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
            disinfo_prob: 0.5,
        }
    }
}

impl BaselineModel for StratifiedBaseline {
    fn train(&mut self, samples: &[Sample]) {
        let dist = Dataset::label_distribution(samples);

        let disinfo_count = *dist.get(&Label::Disinformation).unwrap_or(&0);
        let authentic_count = *dist.get(&Label::Authentic).unwrap_or(&0);
        let total = disinfo_count + authentic_count;

        self.disinfo_prob = disinfo_count as f64 / total as f64;
        self.rng = ChaCha8Rng::seed_from_u64(self.seed);
    }

    fn predict(&self, _sample: &Sample) -> Prediction {
        let mut rng = self.rng.clone();
        let rand_val: f64 = rng.gen();

        Prediction {
            label: if rand_val < self.disinfo_prob {
                Label::Disinformation
            } else {
                Label::Authentic
            },
            probability: self.disinfo_prob,
        }
    }

    fn predict_batch(&self, samples: &[Sample]) -> Vec<Prediction> {
        let mut rng = self.rng.clone();
        samples
            .iter()
            .map(|_| {
                let rand_val: f64 = rng.gen();
                Prediction {
                    label: if rand_val < self.disinfo_prob {
                        Label::Disinformation
                    } else {
                        Label::Authentic
                    },
                    probability: self.disinfo_prob,
                }
            })
            .collect()
    }

    fn name(&self) -> &str {
        "Stratified"
    }

    fn description(&self) -> &str {
        "Predicts proportionally to training class distribution"
    }
}

/// TF-IDF baseline with keyword matching
///
/// Uses simple term frequency to detect disinformation-associated terms
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TfIdfBaseline {
    /// Word frequencies in disinformation class
    disinfo_tf: HashMap<String, f64>,
    /// Word frequencies in authentic class
    authentic_tf: HashMap<String, f64>,
    /// Document frequencies across all documents
    df: HashMap<String, usize>,
    /// Total documents in training
    n_docs: usize,
    /// Prior probability of disinformation
    prior_disinfo: f64,
    /// Vocabulary
    vocab: Vec<String>,
}

impl TfIdfBaseline {
    pub fn new() -> Self {
        Self::default()
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 2) // Skip short words
            .map(|s| s.to_string())
            .collect()
    }

    fn compute_tfidf(&self, text: &str) -> HashMap<String, f64> {
        let tokens = Self::tokenize(text);
        let mut tf: HashMap<String, usize> = HashMap::new();

        for token in &tokens {
            *tf.entry(token.clone()).or_insert(0) += 1;
        }

        let doc_len = tokens.len() as f64;
        let mut tfidf = HashMap::new();

        for (term, count) in tf {
            let tf_val = count as f64 / doc_len.max(1.0);
            let df_val = *self.df.get(&term).unwrap_or(&1);
            let idf = (self.n_docs as f64 / df_val as f64).ln() + 1.0;
            tfidf.insert(term, tf_val * idf);
        }

        tfidf
    }
}

impl TfIdfBaseline {
    /// Compute term contributions to the prediction score
    fn compute_term_contributions(&self, text: &str) -> Vec<(String, f64)> {
        let tfidf = self.compute_tfidf(text);
        let smoothing = 1e-10;

        let mut contributions: Vec<(String, f64)> = tfidf
            .iter()
            .map(|(term, weight)| {
                let disinfo_prob = self.disinfo_tf.get(term).copied().unwrap_or(smoothing);
                let authentic_prob = self.authentic_tf.get(term).copied().unwrap_or(smoothing);

                // Contribution is the difference in log probabilities, weighted by TF-IDF
                let contribution = weight * (disinfo_prob.ln() - authentic_prob.ln());
                (term.clone(), contribution)
            })
            .collect();

        // Sort by absolute contribution (most important first)
        contributions.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal));
        contributions
    }
}

impl BaselineModel for TfIdfBaseline {
    fn train(&mut self, samples: &[Sample]) {
        self.disinfo_tf.clear();
        self.authentic_tf.clear();
        self.df.clear();
        self.vocab.clear();

        let mut disinfo_word_counts: HashMap<String, usize> = HashMap::new();
        let mut authentic_word_counts: HashMap<String, usize> = HashMap::new();
        let mut disinfo_total_words = 0usize;
        let mut authentic_total_words = 0usize;
        let mut n_disinfo = 0usize;
        let mut n_authentic = 0usize;

        for sample in samples {
            let tokens = Self::tokenize(&sample.text);

            // Track document frequency
            let unique_tokens: std::collections::HashSet<_> = tokens.iter().cloned().collect();
            for token in &unique_tokens {
                *self.df.entry(token.clone()).or_insert(0) += 1;
            }

            // Track term frequency per class
            match sample.label {
                Label::Disinformation => {
                    n_disinfo += 1;
                    for token in &tokens {
                        *disinfo_word_counts.entry(token.clone()).or_insert(0) += 1;
                        disinfo_total_words += 1;
                    }
                }
                Label::Authentic => {
                    n_authentic += 1;
                    for token in &tokens {
                        *authentic_word_counts.entry(token.clone()).or_insert(0) += 1;
                        authentic_total_words += 1;
                    }
                }
                _ => {}
            }
        }

        self.n_docs = n_disinfo + n_authentic;
        self.prior_disinfo = n_disinfo as f64 / self.n_docs as f64;

        // Normalize to term frequencies
        for (term, count) in disinfo_word_counts {
            self.disinfo_tf
                .insert(term.clone(), count as f64 / disinfo_total_words as f64);
            self.vocab.push(term);
        }

        for (term, count) in authentic_word_counts {
            self.authentic_tf
                .insert(term, count as f64 / authentic_total_words as f64);
        }

        // Build vocabulary
        self.vocab.sort();
        self.vocab.dedup();
    }

    fn predict(&self, sample: &Sample) -> Prediction {
        let tfidf = self.compute_tfidf(&sample.text);

        // Naive Bayes-style scoring
        let mut disinfo_score = self.prior_disinfo.ln();
        let mut authentic_score = (1.0 - self.prior_disinfo).ln();

        let smoothing = 1e-10;

        for (term, weight) in &tfidf {
            let disinfo_prob = self.disinfo_tf.get(term).copied().unwrap_or(smoothing);
            let authentic_prob = self.authentic_tf.get(term).copied().unwrap_or(smoothing);

            disinfo_score += weight * disinfo_prob.ln();
            authentic_score += weight * authentic_prob.ln();
        }

        // Convert log scores to probability
        let max_score = disinfo_score.max(authentic_score);
        let disinfo_exp = (disinfo_score - max_score).exp();
        let authentic_exp = (authentic_score - max_score).exp();
        let total = disinfo_exp + authentic_exp;

        let probability = disinfo_exp / total;

        Prediction {
            label: if probability > 0.5 {
                Label::Disinformation
            } else {
                Label::Authentic
            },
            probability,
        }
    }

    fn predict_with_explanation(&self, sample: &Sample) -> PredictionWithExplanation {
        let pred = self.predict(sample);
        let contributions = self.compute_term_contributions(&sample.text);

        let mut builder = ExplanationBuilder::new(pred.label, pred.probability);

        // Add top contributing terms as evidence
        let top_n = 5;
        for (term, contribution) in contributions.iter().take(top_n) {
            let evidence_type = if *contribution > 0.0 {
                EvidenceType::LinguisticPattern
            } else {
                EvidenceType::LinguisticPattern
            };

            let direction = if *contribution > 0.0 {
                "towards disinformation"
            } else {
                "towards authentic"
            };

            builder = builder.with_evidence(Evidence {
                evidence_type,
                description: format!("Term '{}' contributes {} (score: {:.4})", term, direction, contribution),
                weight: contribution.abs(),
                span: None,
                source: "tfidf_classifier".to_string(),
            });
        }

        // Add reasoning step
        let top_disinfo: Vec<_> = contributions.iter().filter(|(_, c)| *c > 0.0).take(3).collect();
        let top_authentic: Vec<_> = contributions.iter().filter(|(_, c)| *c < 0.0).take(3).collect();

        builder = builder.with_reasoning_step(
            "naive_bayes_classification",
            vec![
                format!("Prior P(disinfo) = {:.3}", self.prior_disinfo),
                format!("Top disinfo terms: {:?}", top_disinfo.iter().map(|(t, _)| t).collect::<Vec<_>>()),
                format!("Top authentic terms: {:?}", top_authentic.iter().map(|(t, _)| t).collect::<Vec<_>>()),
            ],
            &format!(
                "Combined evidence yields P(disinfo) = {:.3}",
                pred.probability
            ),
            pred.probability,
        );

        // Add feature attributions
        let attributions: HashMap<String, f64> = contributions.iter().cloned().collect();
        builder = builder.with_attributions(attributions);

        // Add uncertainty factors
        if contributions.is_empty() {
            builder = builder.with_uncertainty("No recognized vocabulary terms in text");
        } else if pred.probability > 0.4 && pred.probability < 0.6 {
            builder = builder.with_uncertainty("Prediction confidence is near decision boundary");
        }

        PredictionWithExplanation {
            prediction: pred,
            explanation: builder.build(),
        }
    }

    fn name(&self) -> &str {
        "TF-IDF"
    }

    fn description(&self) -> &str {
        "TF-IDF weighted Naive Bayes classifier"
    }

    fn supports_explanations(&self) -> bool {
        true
    }
}

/// Keyword-based baseline using hand-crafted disinformation indicators
#[derive(Debug, Clone)]
pub struct KeywordBaseline {
    /// Keywords strongly associated with disinformation
    disinfo_keywords: Vec<String>,
    /// Keywords strongly associated with authentic content
    authentic_keywords: Vec<String>,
    /// Weight per keyword match
    keyword_weight: f64,
}

impl KeywordBaseline {
    pub fn new() -> Self {
        Self {
            disinfo_keywords: vec![
                // Sensationalism
                "breaking".to_string(),
                "shocking".to_string(),
                "unbelievable".to_string(),
                "urgent".to_string(),
                "exclusive".to_string(),
                // Conspiracy language
                "conspiracy".to_string(),
                "coverup".to_string(),
                "secret".to_string(),
                "hidden".to_string(),
                "exposed".to_string(),
                // Manipulative
                "miracle".to_string(),
                "cure".to_string(),
                "doctors hate".to_string(),
                "one weird trick".to_string(),
                // Emotional manipulation
                "outraged".to_string(),
                "horrifying".to_string(),
                "terrifying".to_string(),
                // Urgency/scarcity
                "before it's deleted".to_string(),
                "share now".to_string(),
                "viral".to_string(),
            ],
            authentic_keywords: vec![
                // Attribution
                "according to".to_string(),
                "study shows".to_string(),
                "research".to_string(),
                "peer-reviewed".to_string(),
                "published".to_string(),
                // Sourcing
                "official".to_string(),
                "spokesperson".to_string(),
                "confirmed".to_string(),
                "verified".to_string(),
                // Nuance
                "however".to_string(),
                "although".to_string(),
                "experts say".to_string(),
                "evidence suggests".to_string(),
                // Institutions
                "university".to_string(),
                "institute".to_string(),
                "journal".to_string(),
            ],
            keyword_weight: 0.1,
        }
    }
}

impl Default for KeywordBaseline {
    fn default() -> Self {
        Self::new()
    }
}

impl KeywordBaseline {
    /// Get matched keywords from text
    fn get_keyword_matches(&self, text: &str) -> (Vec<String>, Vec<String>) {
        let text_lower = text.to_lowercase();

        let disinfo_matches: Vec<String> = self
            .disinfo_keywords
            .iter()
            .filter(|kw| text_lower.contains(&kw.to_lowercase()))
            .cloned()
            .collect();

        let authentic_matches: Vec<String> = self
            .authentic_keywords
            .iter()
            .filter(|kw| text_lower.contains(&kw.to_lowercase()))
            .cloned()
            .collect();

        (disinfo_matches, authentic_matches)
    }
}

impl BaselineModel for KeywordBaseline {
    fn train(&mut self, _samples: &[Sample]) {
        // Keyword baseline uses fixed keyword lists, no training needed
    }

    fn predict(&self, sample: &Sample) -> Prediction {
        let (disinfo_matches, authentic_matches) = self.get_keyword_matches(&sample.text);

        // Score based on keyword matches
        let disinfo_score = disinfo_matches.len() as f64 * self.keyword_weight;
        let authentic_score = authentic_matches.len() as f64 * self.keyword_weight;

        // Convert to probability with sigmoid
        let diff = disinfo_score - authentic_score;
        let probability = 1.0 / (1.0 + (-diff * 5.0).exp()); // Scaled sigmoid

        Prediction {
            label: if probability > 0.5 {
                Label::Disinformation
            } else {
                Label::Authentic
            },
            probability,
        }
    }

    fn predict_with_explanation(&self, sample: &Sample) -> PredictionWithExplanation {
        let (disinfo_matches, authentic_matches) = self.get_keyword_matches(&sample.text);
        let pred = self.predict(sample);

        let mut builder = ExplanationBuilder::new(pred.label, pred.probability);

        // Add evidence for disinformation keywords
        for kw in &disinfo_matches {
            builder = builder.with_evidence(Evidence {
                evidence_type: EvidenceType::KeywordMatch,
                description: format!("Disinformation indicator: '{}'", kw),
                weight: self.keyword_weight,
                span: None,
                source: "keyword_baseline".to_string(),
            });
        }

        // Add evidence for authentic keywords
        for kw in &authentic_matches {
            builder = builder.with_evidence(Evidence {
                evidence_type: EvidenceType::KeywordMatch,
                description: format!("Authenticity indicator: '{}'", kw),
                weight: self.keyword_weight,
                span: None,
                source: "keyword_baseline".to_string(),
            });
        }

        // Add reasoning step
        let reasoning = format!(
            "Found {} disinformation indicators and {} authenticity indicators",
            disinfo_matches.len(),
            authentic_matches.len()
        );
        builder = builder.with_reasoning_step(
            "keyword_scoring",
            vec![
                format!("Disinformation keywords: {:?}", disinfo_matches),
                format!("Authentic keywords: {:?}", authentic_matches),
            ],
            &reasoning,
            pred.probability,
        );

        // Add uncertainty if no keywords matched
        if disinfo_matches.is_empty() && authentic_matches.is_empty() {
            builder = builder.with_uncertainty("No indicator keywords found in text");
        }

        // Add feature attributions
        let mut attributions = HashMap::new();
        for kw in &disinfo_matches {
            attributions.insert(format!("kw_disinfo_{}", kw), self.keyword_weight);
        }
        for kw in &authentic_matches {
            attributions.insert(format!("kw_authentic_{}", kw), -self.keyword_weight);
        }
        builder = builder.with_attributions(attributions);

        PredictionWithExplanation {
            prediction: pred,
            explanation: builder.build(),
        }
    }

    fn name(&self) -> &str {
        "Keyword"
    }

    fn description(&self) -> &str {
        "Hand-crafted keyword matching baseline"
    }

    fn supports_explanations(&self) -> bool {
        true
    }
}

/// Factory function to create all baseline models
pub fn all_baselines(seed: u64) -> Vec<Box<dyn BaselineModel>> {
    vec![
        Box::new(RandomBaseline::new(seed)),
        Box::new(MajorityBaseline::new()),
        Box::new(StratifiedBaseline::new(seed)),
        Box::new(TfIdfBaseline::new()),
        Box::new(KeywordBaseline::new()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_samples() -> Vec<Sample> {
        vec![
            Sample {
                id: "1".to_string(),
                text: "BREAKING: Shocking new discovery scientists don't want you to know".to_string(),
                label: Label::Disinformation,
                original_label: Some("fake".to_string()),
                metadata: HashMap::new(),
            },
            Sample {
                id: "2".to_string(),
                text: "According to a peer-reviewed study published in Nature, researchers found evidence...".to_string(),
                label: Label::Authentic,
                original_label: Some("real".to_string()),
                metadata: HashMap::new(),
            },
            Sample {
                id: "3".to_string(),
                text: "This miracle cure that doctors hate will change your life".to_string(),
                label: Label::Disinformation,
                original_label: Some("fake".to_string()),
                metadata: HashMap::new(),
            },
            Sample {
                id: "4".to_string(),
                text: "The university spokesperson confirmed the research findings".to_string(),
                label: Label::Authentic,
                original_label: Some("real".to_string()),
                metadata: HashMap::new(),
            },
        ]
    }

    #[test]
    fn test_random_baseline() {
        let samples = create_test_samples();
        let mut baseline = RandomBaseline::new(42);
        baseline.train(&samples);

        let predictions = baseline.predict_batch(&samples);
        assert_eq!(predictions.len(), 4);

        // All probabilities should be between 0 and 1
        for pred in &predictions {
            assert!(pred.probability >= 0.0 && pred.probability <= 1.0);
        }
    }

    #[test]
    fn test_majority_baseline() {
        let samples = create_test_samples();
        let mut baseline = MajorityBaseline::new();
        baseline.train(&samples);

        // With balanced dataset, should pick one class consistently
        let predictions = baseline.predict_batch(&samples);

        let first_label = &predictions[0].label;
        for pred in &predictions {
            assert_eq!(&pred.label, first_label);
        }
    }

    #[test]
    fn test_keyword_baseline() {
        let samples = create_test_samples();
        let mut baseline = KeywordBaseline::new();
        baseline.train(&samples);

        // Should detect obvious disinformation
        let disinfo_pred = baseline.predict(&samples[0]);
        assert_eq!(disinfo_pred.label, Label::Disinformation);
        assert!(disinfo_pred.probability > 0.5);

        // Should detect obvious authentic content
        let authentic_pred = baseline.predict(&samples[1]);
        assert_eq!(authentic_pred.label, Label::Authentic);
        assert!(authentic_pred.probability < 0.5);
    }

    #[test]
    fn test_tfidf_baseline() {
        let samples = create_test_samples();
        let mut baseline = TfIdfBaseline::new();
        baseline.train(&samples);

        // Should learn from training data
        assert!(!baseline.vocab.is_empty());
        assert!(!baseline.disinfo_tf.is_empty());
        assert!(!baseline.authentic_tf.is_empty());

        let predictions = baseline.predict_batch(&samples);
        assert_eq!(predictions.len(), 4);
    }

    #[test]
    fn test_all_baselines() {
        let baselines = all_baselines(42);
        assert_eq!(baselines.len(), 5);

        let names: Vec<_> = baselines.iter().map(|b| b.name()).collect();
        assert!(names.contains(&"Random"));
        assert!(names.contains(&"Majority"));
        assert!(names.contains(&"Stratified"));
        assert!(names.contains(&"TF-IDF"));
        assert!(names.contains(&"Keyword"));
    }
}
