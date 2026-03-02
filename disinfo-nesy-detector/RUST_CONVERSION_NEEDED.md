# Go → Rust Conversion Required

This repository needs conversion from Go to Rust per RSR policy.

## Current Components
- **NATS JetStream**: Message streaming → Use `async-nats` crate
- **ONNX Runtime**: ML inference → Use `ort` crate
- **Souffle Wrapper**: Datalog → Use `crepe` or FFI to Souffle
- **Prometheus**: Metrics → Use `prometheus` crate
- **Protobuf**: Serialization → Use `prost` crate

## Rust Equivalent Crates
```toml
[dependencies]
async-nats = "0.33"
ort = "2.0"
prometheus = "0.13"
prost = "0.12"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
```

## Conversion Steps
1. Create Cargo.toml with above dependencies
2. Convert pkg/onnx_wrapper → Rust ONNX bindings
3. Convert pkg/souffle_wrapper → Rust Datalog (crepe or FFI)
4. Convert cmd/main.go → Rust async main with tokio
5. Generate Rust protobuf from .proto files
6. Test with NATS and ONNX model

## Priority
High - ML inference service needs Rust's safety guarantees.
