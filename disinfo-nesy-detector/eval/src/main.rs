// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Evaluation pipeline CLI for Neuro-Symbolic Disinformation Detector
//!
//! Usage:
//!   eval-pipeline --dataset synthetic --seed 42
//!   eval-pipeline --dataset liar --path ./datasets/liar --split test

use anyhow::Result;
use clap::Parser;
use disinfo_eval::pipeline::{EvaluationConfig, EvaluationPipeline};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "eval-pipeline")]
#[command(about = "Evaluate disinformation detection models")]
#[command(version)]
struct Args {
    /// Dataset to evaluate on (synthetic, liar, isot)
    #[arg(short, long, default_value = "synthetic")]
    dataset: String,

    /// Path to dataset directory
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Random seed for reproducibility
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Evaluation split (train, validation, test)
    #[arg(long, default_value = "test")]
    split: String,

    /// Specific baselines to run (comma-separated, empty = all)
    #[arg(short, long)]
    baselines: Option<String>,

    /// Output directory for results
    #[arg(short, long, default_value = "eval/results")]
    output: PathBuf,

    /// Output format (json, markdown, both)
    #[arg(short, long, default_value = "both")]
    format: String,

    /// Generate model cards for each baseline
    #[arg(long, default_value_t = true)]
    model_cards: bool,
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    tracing::info!("Disinformation Detection Evaluation Pipeline");
    tracing::info!("============================================");
    tracing::info!("Dataset: {}", args.dataset);
    tracing::info!("Seed: {}", args.seed);
    tracing::info!("Split: {}", args.split);

    let baseline_names: Vec<String> = args
        .baselines
        .map(|b| b.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let config = EvaluationConfig {
        seed: args.seed,
        dataset_id: args.dataset.clone(),
        dataset_path: args.path.map(|p| p.to_string_lossy().to_string()),
        eval_split: args.split,
        run_baselines: true,
        baseline_names,
        output_dir: args.output.to_string_lossy().to_string(),
    };

    let mut pipeline = EvaluationPipeline::new(config);
    let results = pipeline.run()?;

    // Print summary to console
    println!("\n{}", "=".repeat(70));
    println!("EVALUATION SUMMARY");
    println!("{}", "=".repeat(70));
    println!("\nBest Model: {} (F1={:.4})", results.summary.best_model, results.summary.best_f1);
    println!("\nBaseline Comparison:");
    println!("{:-<70}", "");
    println!("{:<15} {:>10} {:>10} {:>10} {:>10} {:>10}", "Model", "Accuracy", "F1", "MCC", "AUC-ROC", "Explain");
    println!("{:-<70}", "");

    for result in &results.baseline_results {
        let auc = result.metrics.auc_roc.map_or("-".to_string(), |v| format!("{:.4}", v));
        let explain = if result.supports_explanations { "Yes" } else { "-" };
        println!(
            "{:<15} {:>10.4} {:>10.4} {:>10.4} {:>10} {:>10}",
            result.model_name,
            result.metrics.classification.accuracy,
            result.metrics.classification.f1_score,
            result.metrics.classification.mcc,
            auc,
            explain
        );
    }
    println!("{:-<70}", "");

    // Show explainability metrics for models that support it
    let explainable_models: Vec<_> = results.baseline_results.iter()
        .filter(|r| r.supports_explanations && r.explainability_metrics.is_some())
        .collect();

    if !explainable_models.is_empty() {
        println!("\nExplainability Metrics:");
        println!("{:-<70}", "");
        for result in explainable_models {
            if let Some(ref exp_metrics) = result.explainability_metrics {
                println!(
                    "{}: Completeness={:.1}%, Consistency={:.1}%, Evidence={:.1}",
                    result.model_name,
                    exp_metrics.avg_completeness * 100.0,
                    exp_metrics.feature_consistency * 100.0,
           Add pinned toolchains + reproducible build         exp_metrics.avg_evidence_count
                );
            }
        }
        println!("{:-<70}", "");
    }

    // Save outputs
    std::fs::create_dir_all(&args.output)?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

    if args.format == "json" || args.format == "both" {
        let json_path = args.output.join(format!("eval_{}_{}.json", args.dataset, timestamp));
        EvaluationPipeline::save_results(&results, &json_path)?;
        println!("\nJSON results saved to: {}", json_path.display());
    }

    if args.format == "markdown" || args.format == "both" {
        let report = EvaluationPipeline::generate_report(&results);
        let md_path = args.output.join(format!("eval_{}_{}.md", args.dataset, timestamp));
        std::fs::write(&md_path, report)?;
        println!("Markdown report saved to: {}", md_path.display());
    }

    // Generate model cards
    if args.model_cards {
        let cards = EvaluationPipeline::save_model_cards(&results, &args.output)?;
        println!("Model cards saved to: {}/model_cards/ ({} files)", args.output.display(), cards.len());
    }

    println!("\nEvaluation complete!");

    Ok(())
}
