// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2024 Hyperpolymath

//! Dataset download utility for evaluation pipeline
//!
//! Downloads and extracts evaluation datasets:
//! - LIAR: Political fact-checking dataset
//! - ISOT: Fake news article dataset

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "download-datasets")]
#[command(about = "Download evaluation datasets")]
#[command(version)]
struct Args {
    /// Datasets to download (comma-separated: liar,isot or 'all')
    #[arg(short, long, default_value = "all")]
    datasets: String,

    /// Output directory
    #[arg(short, long, default_value = "eval/datasets")]
    output: PathBuf,

    /// Skip verification of checksums
    #[arg(long)]
    skip_verify: bool,

    /// Force re-download even if files exist
    #[arg(short, long)]
    force: bool,
}

struct DatasetDownload {
    id: &'static str,
    name: &'static str,
    url: &'static str,
    filename: &'static str,
    sha256: Option<&'static str>,
    extract_type: ExtractType,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum ExtractType {
    Zip,
    TarGz,
    None,
}

const DATASETS: &[DatasetDownload] = &[
    DatasetDownload {
        id: "liar",
        name: "LIAR Dataset",
        url: "https://www.cs.ucsb.edu/~william/data/liar_dataset.zip",
        filename: "liar_dataset.zip",
        sha256: None, // Will verify file structure instead
        extract_type: ExtractType::Zip,
    },
    // Note: ISOT requires manual download due to access restrictions
    // This is a placeholder for when direct download becomes available
];

fn download_file(url: &str, output_path: &Path) -> Result<()> {
    tracing::info!("Downloading from: {}", url);

    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()?
        .get(url)
        .send()
        .context("Failed to send request")?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed with status: {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = File::create(output_path).context("Failed to create output file")?;
    let content = response.bytes().context("Failed to read response")?;

    pb.set_position(content.len() as u64);
    file.write_all(&content)?;

    pb.finish_with_message("Downloaded");
    Ok(())
}

fn verify_sha256(path: &Path, expected: &str) -> Result<bool> {
    tracing::info!("Verifying checksum...");

    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hex::encode(hasher.finalize());
    let matches = result == expected;

    if !matches {
        tracing::warn!("Checksum mismatch: expected {}, got {}", expected, result);
    } else {
        tracing::info!("Checksum verified: {}", result);
    }

    Ok(matches)
}

fn extract_zip(archive_path: &Path, output_dir: &Path) -> Result<()> {
    tracing::info!("Extracting ZIP archive...");

    let file = File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let pb = ProgressBar::new(archive.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Extracting: [{wide_bar:.cyan/blue}] {pos}/{len}")
            .unwrap(),
    );

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.mangled_name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        pb.inc(1);
    }

    pb.finish_with_message("Extracted");
    Ok(())
}

fn extract_tar_gz(archive_path: &Path, output_dir: &Path) -> Result<()> {
    tracing::info!("Extracting TAR.GZ archive...");

    let file = File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(output_dir)?;

    tracing::info!("Extraction complete");
    Ok(())
}

fn download_dataset(dataset: &DatasetDownload, output_dir: &Path, skip_verify: bool, force: bool) -> Result<()> {
    tracing::info!("Processing dataset: {} ({})", dataset.name, dataset.id);

    let dataset_dir = output_dir.join(dataset.id);
    let archive_path = output_dir.join(dataset.filename);

    // Check if already extracted
    if dataset_dir.exists() && !force {
        tracing::info!("Dataset directory already exists: {}", dataset_dir.display());
        tracing::info!("Use --force to re-download");
        return Ok(());
    }

    // Download if needed
    if !archive_path.exists() || force {
        download_file(dataset.url, &archive_path)?;
    } else {
        tracing::info!("Archive already exists: {}", archive_path.display());
    }

    // Verify checksum
    if !skip_verify {
        if let Some(expected_hash) = dataset.sha256 {
            if !verify_sha256(&archive_path, expected_hash)? {
                anyhow::bail!("Checksum verification failed for {}", dataset.filename);
            }
        }
    }

    // Create output directory
    std::fs::create_dir_all(&dataset_dir)?;

    // Extract
    match dataset.extract_type {
        ExtractType::Zip => extract_zip(&archive_path, &dataset_dir)?,
        ExtractType::TarGz => extract_tar_gz(&archive_path, &dataset_dir)?,
        ExtractType::None => {
            // Just copy the file
            std::fs::copy(&archive_path, dataset_dir.join(dataset.filename))?;
        }
    }

    tracing::info!("Dataset ready: {}", dataset_dir.display());
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    tracing::info!("Dataset Download Utility");
    tracing::info!("========================");

    std::fs::create_dir_all(&args.output)?;

    let requested: Vec<&str> = if args.datasets == "all" {
        DATASETS.iter().map(|d| d.id).collect()
    } else {
        args.datasets.split(',').map(|s| s.trim()).collect()
    };

    for dataset in DATASETS {
        if requested.contains(&dataset.id) {
            if let Err(e) = download_dataset(dataset, &args.output, args.skip_verify, args.force) {
                tracing::error!("Failed to download {}: {}", dataset.id, e);

                // Provide manual download instructions
                if dataset.id == "liar" {
                    tracing::info!("Manual download instructions for LIAR:");
                    tracing::info!("  1. Visit: https://www.cs.ucsb.edu/~william/data/liar_dataset.zip");
                    tracing::info!("  2. Download and extract to: {}/liar/", args.output.display());
                }
            }
        }
    }

    // Print instructions for datasets requiring manual download
    if requested.iter().any(|&d| d == "isot") {
        println!("\n{}", "=".repeat(60));
        println!("ISOT Dataset - Manual Download Required");
        println!("{}", "=".repeat(60));
        println!("\nThe ISOT dataset requires manual download:");
        println!("  1. Visit: https://onlineacademiccommunity.uvic.ca/isot/");
        println!("  2. Request access to the dataset");
        println!("  3. Download 'News-dataset.zip'");
        println!("  4. Extract to: {}/isot/", args.output.display());
        println!("  5. Ensure files are named: Fake.csv and True.csv\n");
    }

    println!("\n{}", "=".repeat(60));
    println!("Dataset Preparation Complete");
    println!("{}", "=".repeat(60));
    println!("\nAvailable datasets in {}:", args.output.display());

    for entry in std::fs::read_dir(&args.output)? {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                println!("  - {}", entry.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}
