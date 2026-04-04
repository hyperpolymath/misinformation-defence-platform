# TEST-NEEDS.md — Test Coverage Status

<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <6759885+hyperpolymath@users.noreply.github.com> -->

## CRG Grade: C — ACHIEVED 2026-04-04

## CRG Grade: **C** (achieved 2026-04-04)

### Coverage Matrix

| Test Category        | Location                                                         | Count | Status |
|----------------------|------------------------------------------------------------------|-------|--------|
| Unit tests (inline)  | `disinfo-nesy-detector/eval/src/baselines.rs`                    | 5     | PASS   |
| Unit tests (inline)  | `disinfo-nesy-detector/eval/src/datasets.rs`                     | 3     | PASS   |
| Unit tests (inline)  | `disinfo-nesy-detector/eval/src/explainability.rs`               | 4     | PASS   |
| Unit tests (inline)  | `disinfo-nesy-detector/eval/src/metrics.rs`                      | 7     | PASS   |
| Unit tests (inline)  | `disinfo-nesy-detector/eval/src/pipeline.rs`                     | 3     | PASS   |
| Smoke / build        | `cargo build` + `cargo test` (all crates)                        | —     | PASS   |
| P2P property tests   | `disinfo-nesy-detector/eval/tests/property_tests.rs`             | 11    | PASS   |
| E2E tests            | `disinfo-nesy-detector/eval/tests/e2e_tests.rs`                  | 8     | PASS   |
| Aspect tests         | `disinfo-nesy-detector/eval/tests/aspect_tests.rs`               | 13    | PASS   |
| Benchmarks           | `disinfo-nesy-detector/eval/benches/detection_bench.rs`          | 12    | BASELINED |

**Total integration tests:** 57 (25 unit + 11 property + 8 E2E + 13 aspect)

### Benchmark Baselines (2026-04-04, release profile)

| Benchmark                        | Median time   |
|----------------------------------|---------------|
| keyword/single_claim             | ~10.9 µs      |
| tfidf/single_claim               | ~69.6 µs      |
| tfidf/batch/10                   | ~1.26 ms      |
| tfidf/batch/100                  | ~6.1 ms       |
| tfidf/batch/1000                 | ~72 ms        |
| keyword/batch/1000               | ~5.0 ms       |
| tfidf/train_1000_samples         | ~2.1 ms       |
| explain/keyword/disinfo          | ~23.4 µs      |
| explain/tfidf/disinfo            | ~64.3 µs      |
| explain/tfidf/authentic          | ~109.4 µs     |

All single-claim detections are well within the 100 ms aspect test limit.

### Property Invariants Verified

- Confidence scores always in `[0.0, 1.0]` for all models and any text input
- KeywordBaseline and MajorityBaseline are deterministic (same input → same output)
- Majority class prediction is constant regardless of input
- Keyword explanation is non-empty for texts with matched keywords
- Label binary round-trip (Disinformation/Authentic) is stable
- `predict_batch` matches `predict` for all inputs (TF-IDF)
- Synthetic dataset split sizes always sum to total
- ExplanationBuilder preserves confidence exactly

### Aspect (Cross-Cutting) Coverage

- Security: null bytes, oversized (100k chars), Unicode combining marks,
  bidi overrides, non-ASCII symbols, repeated keywords — all handled without panic
- Performance: single-claim detection verified < 100 ms for both models
- Error handling: empty string, whitespace-only, single char — all handled
- Contract: label-probability consistency enforced for KeywordBaseline
- Robustness: mixed adversarial batch (8 edge-case variants) completes cleanly

### What is Stubbed / Not Yet Implemented

The following are not yet testable because the corresponding source is not
implemented (only the eval pipeline exists; the core ONNX/neuro-symbolic
detector is disabled pending model availability):

- `disinfo-nesy-detector/src/onnx_wrapper.rs` — ONNX runtime is commented out
- `disinfo-nesy-detector/src/souffle_wrapper.rs` — Soufflé Datalog integration
- `disinfo-nesy-detector/src/main.rs` — NATS JetStream detector service
- Contract tests against live NATS — require running NATS server
- Reflexive tests on model-card JSON schema — schema not yet published

These should be added at CRG B when the core detector is implemented.

### Next Steps (CRG B)

- [ ] Implement and test ONNX wrapper once model is available
- [ ] Contract tests against NATS JetStream API
- [ ] Reflexive tests: load saved eval results and re-verify metrics
- [ ] Mutation testing baseline
- [ ] CI integration: add `cargo test && cargo bench --no-run` to quality.yml
