// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Dataset loading and preprocessing for disinformation detection evaluation

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Binary label for disinformation detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Label {
    /// Content identified as disinformation/fake
    Disinformation,
    /// Content identified as authentic/real
    Authentic,
    /// Uncertain or insufficient information
    Uncertain,
}

impl Label {
    /// Convert to numeric value for metrics calculation
    pub fn to_binary(&self) -> Option<u8> {
        match self {
            Label::Disinformation => Some(1),
            Label::Authentic => Some(0),
            Label::Uncertain => None,
        }
    }

    /// Create from binary prediction (1 = disinformation, 0 = authentic)
    pub fn from_binary(value: u8) -> Self {
        if value == 1 {
            Label::Disinformation
        } else {
            Label::Authentic
        }
    }
}

/// A single sample from a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    /// Unique identifier
    pub id: String,
    /// Text content to classify
    pub text: String,
    /// Ground truth label
    pub label: Label,
    /// Original fine-grained label (if available)
    pub original_label: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Configuration for loading a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String,
    pub format: String,
    pub labels: Vec<String>,
    pub binary_mapping: BinaryMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryMapping {
    pub disinformation: Vec<String>,
    pub authentic: Vec<String>,
    #[serde(default)]
    pub uncertain: Vec<String>,
}

impl BinaryMapping {
    /// Map original label to binary label
    pub fn map_label(&self, original: &str) -> Label {
        let original_lower = original.to_lowercase();
        if self.disinformation.iter().any(|l| l.to_lowercase() == original_lower) {
            Label::Disinformation
        } else if self.authentic.iter().any(|l| l.to_lowercase() == original_lower) {
            Label::Authentic
        } else {
            Label::Uncertain
        }
    }
}

/// A loaded dataset ready for evaluation
#[derive(Debug, Clone)]
pub struct Dataset {
    pub config: DatasetConfig,
    pub train: Vec<Sample>,
    pub validation: Vec<Sample>,
    pub test: Vec<Sample>,
}

impl Dataset {
    /// Load LIAR dataset from directory
    pub fn load_liar(data_dir: &Path) -> Result<Self> {
        let config = DatasetConfig {
            id: "liar".to_string(),
            name: "LIAR Dataset".to_string(),
            description: "Benchmark dataset for fake news detection with 6-class labels".to_string(),
            source: "https://www.cs.ucsb.edu/~william/data/liar_dataset.zip".to_string(),
            format: "tsv".to_string(),
            labels: vec![
                "pants-fire".to_string(),
                "false".to_string(),
                "barely-true".to_string(),
                "half-true".to_string(),
                "mostly-true".to_string(),
                "true".to_string(),
            ],
            binary_mapping: BinaryMapping {
                disinformation: vec!["pants-fire".to_string(), "false".to_string(), "barely-true".to_string()],
                authentic: vec!["half-true".to_string(), "mostly-true".to_string(), "true".to_string()],
                uncertain: vec![],
            },
        };

        let train = Self::load_liar_split(data_dir.join("train.tsv"), &config.binary_mapping)?;
        let validation = Self::load_liar_split(data_dir.join("valid.tsv"), &config.binary_mapping)?;
        let test = Self::load_liar_split(data_dir.join("test.tsv"), &config.binary_mapping)?;

        Ok(Self {
            config,
            train,
            validation,
            test,
        })
    }

    fn load_liar_split(path: PathBuf, mapping: &BinaryMapping) -> Result<Vec<Sample>> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open LIAR file: {}", path.display()))?;
        let reader = BufReader::new(file);
        let mut samples = Vec::new();

        for (idx, line) in reader.lines().enumerate() {
            let line = line.with_context(|| format!("Failed to read line {} in {}", idx, path.display()))?;
            let fields: Vec<&str> = line.split('\t').collect();

            if fields.len() < 3 {
                tracing::warn!("Skipping malformed line {} in {}: insufficient fields", idx, path.display());
                continue;
            }

            let id = fields[0].to_string();
            let original_label = fields[1].to_string();
            let text = fields[2].to_string();
            let label = mapping.map_label(&original_label);

            let mut metadata = HashMap::new();
            if fields.len() > 3 { metadata.insert("subject".to_string(), fields[3].to_string()); }
            if fields.len() > 4 { metadata.insert("speaker".to_string(), fields[4].to_string()); }
            if fields.len() > 5 { metadata.insert("job".to_string(), fields[5].to_string()); }
            if fields.len() > 6 { metadata.insert("state".to_string(), fields[6].to_string()); }
            if fields.len() > 7 { metadata.insert("party".to_string(), fields[7].to_string()); }
            if fields.len() > 13 { metadata.insert("context".to_string(), fields[13].to_string()); }

            samples.push(Sample {
                id,
                text,
                label,
                original_label: Some(original_label),
                metadata,
            });
        }

        Ok(samples)
    }

    /// Load ISOT dataset from directory
    pub fn load_isot(data_dir: &Path) -> Result<Self> {
        let config = DatasetConfig {
            id: "isot".to_string(),
            name: "ISOT Fake News Dataset".to_string(),
            description: "Large-scale dataset with real and fake news articles".to_string(),
            source: "https://onlineacademiccommunity.uvic.ca/isot/".to_string(),
            format: "csv".to_string(),
            labels: vec!["fake".to_string(), "real".to_string()],
            binary_mapping: BinaryMapping {
                disinformation: vec!["fake".to_string()],
                authentic: vec!["real".to_string()],
                uncertain: vec![],
            },
        };

        let fake_samples = Self::load_isot_csv(data_dir.join("Fake.csv"), Label::Disinformation, "fake")?;
        let real_samples = Self::load_isot_csv(data_dir.join("True.csv"), Label::Authentic, "real")?;

        // Combine and split into train/val/test (80/10/10)
        let mut all_samples: Vec<Sample> = fake_samples.into_iter().chain(real_samples).collect();

        // Deterministic shuffle using sample IDs
        all_samples.sort_by(|a, b| a.id.cmp(&b.id));

        let n = all_samples.len();
        let train_end = (n as f64 * 0.8) as usize;
        let val_end = (n as f64 * 0.9) as usize;

        let test = all_samples.split_off(val_end);
        let validation = all_samples.split_off(train_end);
        let train = all_samples;

        Ok(Self {
            config,
            train,
            validation,
            test,
        })
    }

    fn load_isot_csv(path: PathBuf, label: Label, original_label: &str) -> Result<Vec<Sample>> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open ISOT file: {}", path.display()))?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut samples = Vec::new();

        for (idx, result) in reader.records().enumerate() {
            let record = result.with_context(|| format!("Failed to read record {} in {}", idx, path.display()))?;

            let title = record.get(0).unwrap_or("").to_string();
            let text = record.get(1).unwrap_or("").to_string();
            let subject = record.get(2).unwrap_or("").to_string();
            let date = record.get(3).unwrap_or("").to_string();

            // Combine title and text for classification
            let combined_text = if title.is_empty() {
                text.clone()
            } else {
                format!("{} {}", title, text)
            };

            let id = format!("{}_{}", original_label, idx);

            let mut metadata = HashMap::new();
            metadata.insert("title".to_string(), title);
            metadata.insert("subject".to_string(), subject);
            metadata.insert("date".to_string(), date);

            samples.push(Sample {
                id,
                text: combined_text,
                label,
                original_label: Some(original_label.to_string()),
                metadata,
            });
        }

        Ok(samples)
    }

    /// Load a synthetic test dataset for development/testing
    pub fn load_synthetic(size: usize, seed: u64) -> Self {
        use rand::{Rng, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let config = DatasetConfig {
            id: "synthetic".to_string(),
            name: "Synthetic Test Dataset".to_string(),
            description: "Generated dataset for pipeline testing".to_string(),
            source: "generated".to_string(),
            format: "memory".to_string(),
            labels: vec!["disinformation".to_string(), "authentic".to_string()],
            binary_mapping: BinaryMapping {
                disinformation: vec!["disinformation".to_string()],
                authentic: vec!["authentic".to_string()],
                uncertain: vec![],
            },
        };

        let fake_phrases = [
            "BREAKING: Scientists confirm shocking discovery",
            "You won't believe what happened next",
            "The government doesn't want you to know",
            "This miracle cure doctors hate",
            "Secret conspiracy revealed exclusively",
        ];

        let real_phrases = [
            "According to official reports",
            "Research published in peer-reviewed journal",
            "Statement from verified spokesperson",
            "Data analysis shows consistent trends",
            "Expert consensus indicates",
        ];

        let mut samples: Vec<Sample> = (0..size)
            .map(|i| {
                let is_fake = rng.gen_bool(0.5);
                let phrases = if is_fake { &fake_phrases } else { &real_phrases };
                let phrase_idx = rng.gen_range(0..phrases.len());

                Sample {
                    id: format!("synthetic_{}", i),
                    text: format!("{} - sample content {}", phrases[phrase_idx], i),
                    label: if is_fake { Label::Disinformation } else { Label::Authentic },
                    original_label: Some(if is_fake { "fake".to_string() } else { "real".to_string() }),
                    metadata: HashMap::new(),
                }
            })
            .collect();

        // Split 80/10/10
        let n = samples.len();
        let train_end = (n as f64 * 0.8) as usize;
        let val_end = (n as f64 * 0.9) as usize;

        let test = samples.split_off(val_end);
        let validation = samples.split_off(train_end);
        let train = samples;

        Self {
            config,
            train,
            validation,
            test,
        }
    }

    /// Get total number of samples across all splits
    pub fn total_samples(&self) -> usize {
        self.train.len() + self.validation.len() + self.test.len()
    }

    /// Get label distribution for a split
    pub fn label_distribution(samples: &[Sample]) -> HashMap<Label, usize> {
        let mut dist = HashMap::new();
        for sample in samples {
            *dist.entry(sample.label).or_insert(0) += 1;
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_mapping() {
        let mapping = BinaryMapping {
            disinformation: vec!["false".to_string(), "pants-fire".to_string()],
            authentic: vec!["true".to_string(), "mostly-true".to_string()],
            uncertain: vec!["half-true".to_string()],
        };

        assert_eq!(mapping.map_label("false"), Label::Disinformation);
        assert_eq!(mapping.map_label("FALSE"), Label::Disinformation);
        assert_eq!(mapping.map_label("true"), Label::Authentic);
        assert_eq!(mapping.map_label("half-true"), Label::Uncertain);
        assert_eq!(mapping.map_label("unknown"), Label::Uncertain);
    }

    #[test]
    fn test_synthetic_dataset() {
        let dataset = Dataset::load_synthetic(100, 42);

        assert_eq!(dataset.config.id, "synthetic");
        assert_eq!(dataset.total_samples(), 100);
        assert_eq!(dataset.train.len(), 80);
        assert_eq!(dataset.validation.len(), 10);
        assert_eq!(dataset.test.len(), 10);
    }

    #[test]
    fn test_label_distribution() {
        let dataset = Dataset::load_synthetic(1000, 42);
        let dist = Dataset::label_distribution(&dataset.train);

        // With 50% probability, should be roughly balanced
        let disinfo = *dist.get(&Label::Disinformation).unwrap_or(&0);
        let authentic = *dist.get(&Label::Authentic).unwrap_or(&0);

        // Allow 20% deviation from perfect balance
        let expected = dataset.train.len() / 2;
        let tolerance = expected / 5;

        assert!((disinfo as i64 - expected as i64).unsigned_abs() < tolerance as u64);
        assert!((authentic as i64 - expected as i64).unsigned_abs() < tolerance as u64);
    }
}
