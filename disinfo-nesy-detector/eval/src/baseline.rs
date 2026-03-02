// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Standalone baseline model runner
//!
//! Trains and evaluates individual baseline models for quick testing

use anyhow::Result;
use clap::Parser;
use disinfo_eval::baselines::all_baselines;
use disinfo_eval::datasets::{Dataset, Label};
use disinfo_eval::metrics::EvaluationMetrics;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "run-baseline")]
#[command(about = "Run a specific baseline model")]
#[command(version)]
struct Args {
    /// Baseline model to run (Random, Majority, Stratified, TF-IDF, Keyword)
    #[arg(short, long)]
    model: Option<String>,

    /// Dataset to use (synthetic, liar, isot)
    #[arg(short, long, default_value = "synthetic")]
    dataset: String,

    /// Path to dataset directory
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Random seed
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Number of samples for synthetic dataset
    #[arg(short, long, default_value_t = 1000)]
    num_samples: usize,

    /// List available baselines
    #[arg(long)]
    list: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    // List available baselines
    if args.list {
        println!("Available baseline models:");
        println!("--------------------------");
        for baseline in all_baselines(42) {
            println!("  {}: {}", baseline.name(), baseline.description());
        }
        return Ok(());
    }

    // Load dataset
    let dataset = if args.dataset == "synthetic" {
        tracing::info!("Loading synthetic dataset ({} samples, seed={})", args.num_samples, args.seed);
        Dataset::load_synthetic(args.num_samples, args.seed)
    } else if let Some(ref path) = args.path {
        match args.dataset.as_str() {
            "liar" => Dataset::load_liar(path)?,
            "isot" => Dataset::load_isot(path)?,
            _ => {
                tracing::warn!("Unknown dataset, using synthetic");
                Dataset::load_synthetic(args.num_samples, args.seed)
            }
        }
    } else {
        tracing::warn!("No path provided for {}, using synthetic", args.dataset);
        Dataset::load_synthetic(args.num_samples, args.seed)
    };

    println!("\nDataset: {}", dataset.config.name);
    println!("  Train samples: {}", dataset.train.len());
    println!("  Validation samples: {}", dataset.validation.len());
    println!("  Test samples: {}", dataset.test.len());

    let train_dist = Dataset::label_distribution(&dataset.train);
    let test_dist = Dataset::label_distribution(&dataset.test);

    println!("\nTrain distribution:");
    for (label, count) in &train_dist {
        println!("  {:?}: {} ({:.1}%)", label, count, *count as f64 / dataset.train.len() as f64 * 100.0);
    }

    println!("\nTest distribution:");
    for (label, count) in &test_dist {
        println!("  {:?}: {} ({:.1}%)", label, count, *count as f64 / dataset.test.len() as f64 * 100.0);
    }

    // Run baselines
    let baselines = all_baselines(args.seed);
    let filter_model = args.model.as_deref();

    println!("\n{}", "=".repeat(70));
    println!("BASELINE EVALUATION");
    println!("{}", "=".repeat(70));

    for mut baseline in baselines {
        // Filter if specific model requested
        if let Some(filter) = filter_model {
            if !baseline.name().eq_ignore_ascii_case(filter) {
                continue;
            }
        }

        println!("\n## {} ##", baseline.name());
        println!("{}", baseline.description());
        println!("{}", "-".repeat(50));

        // Train
        baseline.train(&dataset.train);

        // Evaluate on test set
        let predictions = baseline.predict_batch(&dataset.test);
        let pred_labels: Vec<Label> = predictions.iter().map(|p| p.label).collect();
        let true_labels: Vec<Label> = dataset.test.iter().map(|s| s.label).collect();
        let probabilities: Vec<f64> = predictions.iter().map(|p| p.probability).collect();

        let metrics = EvaluationMetrics::from_predictions_with_probs(&pred_labels, &true_labels, &probabilities);

        println!("{}", metrics.format());
    }

    println!("\n{}", "=".repeat(70));
    println!("Evaluation complete!");

    Ok(())
}
