// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Protobuf Message Schema — Disinfo Detector Types.
//!
//! This module defines the wire-format data structures used in the 
//! disinformation analysis pipeline. It follows the schema defined 
//! in `proto/analysis.proto`.
//!
//! IMPLEMENTATION: Uses the `prost` crate for high-performance, 
//! idiomatic Rust code generation from Protobuf definitions.

use prost::Message;

/// INPUT SCHEMA: The data provided to the detector for verification.
#[derive(Clone, PartialEq, Message)]
pub struct AnalysisInput {
    #[prost(string, tag = "1")]
    pub content_hash: String,   // SHA-256 identifier for the content.
    #[prost(string, tag = "2")]
    pub content_text: String,   // Raw natural language payload.
    #[prost(string, tag = "3")]
    pub source_id: String,      // Unique ID of the originating author/platform.
    #[prost(string, tag = "4")]
    pub image_url: String,      // Link to any associated visual media.
}

/// OUTPUT SCHEMA: Semantic features extracted by the NSAI pipeline.
#[derive(Clone, PartialEq, Message)]
pub struct NeuralFeatures {
    #[prost(float, tag = "1")]
    pub fakeness_score: f32,    // 0.0 (Safe) to 1.0 (Disinfo).
    #[prost(float, tag = "2")]
    pub emotion_score: f32,     // Intensity of detected emotional manipulation.
    #[prost(bool, tag = "3")]
    pub visual_artifact: bool, // Detection of AI-generated image artifacts.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_input_roundtrip() {
        // VERIFICATION: Ensures that encoding and decoding preserves bit-fidelity.
        let input = AnalysisInput { ... };
        let mut buf = Vec::new();
        input.encode(&mut buf).unwrap();
        let decoded = AnalysisInput::decode(&buf[..]).unwrap();
        assert_eq!(input, decoded);
    }
}
