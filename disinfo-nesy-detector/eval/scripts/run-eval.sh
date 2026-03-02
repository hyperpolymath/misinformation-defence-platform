#!/bin/sh
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2024 Hyperpolymath

# Reproducible Evaluation Pipeline Runner
#
# Usage:
#   ./eval/scripts/run-eval.sh                    # Run with defaults (synthetic dataset)
#   ./eval/scripts/run-eval.sh --dataset liar     # Run with LIAR dataset
#   ./eval/scripts/run-eval.sh --seed 123         # Custom random seed

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EVAL_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$EVAL_DIR")"

# Default configuration
DATASET="synthetic"
SEED="42"
SPLIT="test"
OUTPUT_DIR="${EVAL_DIR}/results"
FORMAT="both"

# Parse arguments
while [ $# -gt 0 ]; do
    case "$1" in
        --dataset|-d)
            DATASET="$2"
            shift 2
            ;;
        --seed|-s)
            SEED="$2"
            shift 2
            ;;
        --split)
            SPLIT="$2"
            shift 2
            ;;
        --output|-o)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --format|-f)
            FORMAT="$2"
            shift 2
            ;;
        --help|-h)
            echo "Reproducible Evaluation Pipeline"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -d, --dataset DATASET   Dataset to use (synthetic, liar, isot) [default: synthetic]"
            echo "  -s, --seed SEED         Random seed for reproducibility [default: 42]"
            echo "      --split SPLIT       Evaluation split (train, validation, test) [default: test]"
            echo "  -o, --output DIR        Output directory for results [default: eval/results]"
            echo "  -f, --format FORMAT     Output format (json, markdown, both) [default: both]"
            echo "  -h, --help              Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                              # Run with synthetic dataset"
            echo "  $0 --dataset liar --seed 123   # Run with LIAR dataset"
            echo "  $0 --format json               # Output only JSON"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Disinformation Detection Evaluation Pipeline                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Configuration:"
echo "  Dataset: ${DATASET}"
echo "  Seed: ${SEED}"
echo "  Split: ${SPLIT}"
echo "  Output: ${OUTPUT_DIR}"
echo "  Format: ${FORMAT}"
echo ""

# Build the evaluation crate
echo "Building evaluation pipeline..."
cd "${EVAL_DIR}"
cargo build --release 2>&1 | head -20

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Set dataset path if not synthetic
DATASET_PATH=""
if [ "$DATASET" != "synthetic" ]; then
    DATASET_PATH="${EVAL_DIR}/datasets/${DATASET}"
    if [ ! -d "$DATASET_PATH" ]; then
        echo ""
        echo "Warning: Dataset directory not found: ${DATASET_PATH}"
        echo "You may need to download the dataset first:"
        echo "  cargo run --release --bin download-datasets -- --datasets ${DATASET}"
        echo ""
        echo "Falling back to synthetic dataset..."
        DATASET="synthetic"
        DATASET_PATH=""
    fi
fi

# Run evaluation
echo ""
echo "Running evaluation..."
echo ""

if [ -n "$DATASET_PATH" ]; then
    cargo run --release --bin eval-pipeline -- \
        --dataset "${DATASET}" \
        --path "${DATASET_PATH}" \
        --seed "${SEED}" \
        --split "${SPLIT}" \
        --output "${OUTPUT_DIR}" \
        --format "${FORMAT}"
else
    cargo run --release --bin eval-pipeline -- \
        --dataset "${DATASET}" \
        --seed "${SEED}" \
        --split "${SPLIT}" \
        --output "${OUTPUT_DIR}" \
        --format "${FORMAT}"
fi

echo ""
echo "Results saved to: ${OUTPUT_DIR}"
echo ""
echo "Done!"
