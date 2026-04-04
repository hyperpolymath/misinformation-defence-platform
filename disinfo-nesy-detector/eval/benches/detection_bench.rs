// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com>

//! Criterion benchmarks for disinfo-eval detection performance.
//!
//! Covers:
//! - Single claim detection throughput (Keyword, TF-IDF)
//! - Batch detection: 10, 100, 1 000 claims (TF-IDF)
//! - Preprocessing throughput (TF-IDF tokenise path)
//! - Explainability generation time (Keyword and TF-IDF with_explanation)
//! - Dataset loading time (synthetic)
//!
//! Run with:
//!   cargo bench --bench detection_bench

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use disinfo_eval::baselines::{BaselineModel, KeywordBaseline, TfIdfBaseline};
use disinfo_eval::datasets::{Dataset, Label, Sample};
use std::collections::HashMap;

// ── Fixture helpers ──────────────────────────────────────────────────────────

fn make_sample(id: &str, text: &str, label: Label) -> Sample {
    Sample {
        id: id.to_string(),
        text: text.to_string(),
        label,
        original_label: None,
        metadata: HashMap::new(),
    }
}

/// A realistic ~200-word news-style article used as the single-claim benchmark input.
const ARTICLE_TEXT: &str = "\
Breaking news: according to official sources, a peer-reviewed study published in \
the journal of applied science has confirmed findings that experts say suggest a \
consistent trend in data analysis. The university spokesperson verified the results, \
noting however that although research indicates a statistically significant effect, \
the evidence suggests further investigation is warranted. Scientists say the institute \
has published corroborating results and the study shows a clear pattern. Official \
channels have confirmed the analysis and the verified spokesperson stated that the \
research team is continuing their peer-reviewed work. Evidence suggests that the \
journal will publish further findings in the coming quarter.\
";

/// A sensationalist text that exercises disinformation keywords.
const DISINFO_TEXT: &str = "\
BREAKING exclusive shocking revelation: doctors hate this one weird miracle cure that \
the government doesn't want you to know. The secret conspiracy is now exposed and going \
viral — share now before it's deleted. Outraged citizens are horrified by this terrifying \
hidden truth. Unbelievable coverup finally revealed in urgent exclusive report.\
";

/// Build a trained TF-IDF model using the 1 000-sample synthetic dataset.
fn build_tfidf() -> TfIdfBaseline {
    let dataset = Dataset::load_synthetic(1_000, 42);
    let mut model = TfIdfBaseline::new();
    model.train(&dataset.train);
    model
}

/// Build a trained Keyword model (no data dependency).
fn build_keyword() -> KeywordBaseline {
    let mut model = KeywordBaseline::new();
    let empty: Vec<Sample> = vec![];
    model.train(&empty);
    model
}

/// Generate a batch of `n` samples alternating between disinfo and authentic text.
fn make_batch(n: usize) -> Vec<Sample> {
    (0..n).map(|i| {
        if i % 2 == 0 {
            make_sample(&format!("b{}", i), DISINFO_TEXT, Label::Disinformation)
        } else {
            make_sample(&format!("b{}", i), ARTICLE_TEXT, Label::Authentic)
        }
    }).collect()
}

// ── Bench 1: Single-claim detection throughput ───────────────────────────────

fn bench_single_claim_keyword(c: &mut Criterion) {
    let model = build_keyword();
    let sample = make_sample("sc", ARTICLE_TEXT, Label::Uncertain);

    c.bench_function("keyword/single_claim", |b| {
        b.iter(|| {
            let pred = model.predict(black_box(&sample));
            black_box(pred.probability)
        })
    });
}

fn bench_single_claim_tfidf(c: &mut Criterion) {
    let model = build_tfidf();
    let sample = make_sample("sc", ARTICLE_TEXT, Label::Uncertain);

    c.bench_function("tfidf/single_claim", |b| {
        b.iter(|| {
            let pred = model.predict(black_box(&sample));
            black_box(pred.probability)
        })
    });
}

fn bench_single_claim_disinfo_keyword(c: &mut Criterion) {
    let model = build_keyword();
    let sample = make_sample("scd", DISINFO_TEXT, Label::Uncertain);

    c.bench_function("keyword/single_claim_disinfo", |b| {
        b.iter(|| {
            let pred = model.predict(black_box(&sample));
            black_box(pred.probability)
        })
    });
}

// ── Bench 2: Batch detection at varying sizes ────────────────────────────────

fn bench_batch_tfidf(c: &mut Criterion) {
    let model = build_tfidf();
    let mut group = c.benchmark_group("tfidf/batch");

    for &size in &[10usize, 100, 1_000] {
        let batch = make_batch(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &batch,
            |b, batch| {
                b.iter(|| {
                    let preds = model.predict_batch(black_box(batch));
                    // Consume the result so the compiler cannot elide the work.
                    black_box(preds.iter().map(|p| p.probability).sum::<f64>())
                })
            },
        );
    }

    group.finish();
}

fn bench_batch_keyword(c: &mut Criterion) {
    let model = build_keyword();
    let mut group = c.benchmark_group("keyword/batch");

    for &size in &[10usize, 100, 1_000] {
        let batch = make_batch(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &batch,
            |b, batch| {
                b.iter(|| {
                    let preds = model.predict_batch(black_box(batch));
                    black_box(preds.iter().map(|p| p.probability).sum::<f64>())
                })
            },
        );
    }

    group.finish();
}

// ── Bench 3: Preprocessing / tokenisation throughput ─────────────────────────

/// Exercises the TF-IDF training path, which exercises the tokeniser over the
/// full training corpus.
fn bench_tfidf_training(c: &mut Criterion) {
    let dataset = Dataset::load_synthetic(1_000, 42);

    c.bench_function("tfidf/train_1000_samples", |b| {
        b.iter(|| {
            let mut model = TfIdfBaseline::new();
            model.train(black_box(&dataset.train));
            black_box(model)
        })
    });
}

// ── Bench 4: Explainability generation time ──────────────────────────────────

fn bench_keyword_explain(c: &mut Criterion) {
    let model = build_keyword();
    let sample_disinfo = make_sample("exp_d", DISINFO_TEXT, Label::Uncertain);
    let sample_auth = make_sample("exp_a", ARTICLE_TEXT, Label::Uncertain);

    let mut group = c.benchmark_group("explain");
    group.bench_function("keyword/disinfo", |b| {
        b.iter(|| {
            let result = model.predict_with_explanation(black_box(&sample_disinfo));
            black_box(result.explanation.evidence.len())
        })
    });
    group.bench_function("keyword/authentic", |b| {
        b.iter(|| {
            let result = model.predict_with_explanation(black_box(&sample_auth));
            black_box(result.explanation.evidence.len())
        })
    });
    group.finish();
}

fn bench_tfidf_explain(c: &mut Criterion) {
    let model = build_tfidf();
    let sample_disinfo = make_sample("exp_d", DISINFO_TEXT, Label::Uncertain);
    let sample_auth = make_sample("exp_a", ARTICLE_TEXT, Label::Uncertain);

    let mut group = c.benchmark_group("explain");
    group.bench_function("tfidf/disinfo", |b| {
        b.iter(|| {
            let result = model.predict_with_explanation(black_box(&sample_disinfo));
            black_box(result.explanation.evidence.len())
        })
    });
    group.bench_function("tfidf/authentic", |b| {
        b.iter(|| {
            let result = model.predict_with_explanation(black_box(&sample_auth));
            black_box(result.explanation.evidence.len())
        })
    });
    group.finish();
}

// ── Bench 5: Dataset loading ──────────────────────────────────────────────────

fn bench_synthetic_dataset_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("dataset/load_synthetic");

    for &size in &[100usize, 1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let dataset = Dataset::load_synthetic(black_box(size), 42);
                    black_box(dataset.total_samples())
                })
            },
        );
    }

    group.finish();
}

// ── Criterion groups ──────────────────────────────────────────────────────────

criterion_group!(
    single_claim_benches,
    bench_single_claim_keyword,
    bench_single_claim_tfidf,
    bench_single_claim_disinfo_keyword,
);

criterion_group!(
    batch_benches,
    bench_batch_tfidf,
    bench_batch_keyword,
);

criterion_group!(
    preprocessing_benches,
    bench_tfidf_training,
);

criterion_group!(
    explain_benches,
    bench_keyword_explain,
    bench_tfidf_explain,
);

criterion_group!(
    dataset_benches,
    bench_synthetic_dataset_load,
);

criterion_main!(
    single_claim_benches,
    batch_benches,
    preprocessing_benches,
    explain_benches,
    dataset_benches,
);
