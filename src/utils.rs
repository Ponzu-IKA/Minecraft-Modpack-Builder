use anyhow::{Context, Result};
use bytes::Bytes;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use serde::de::Error;
use std::time::Duration;
use std::{
    fs::{self, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::{
    ZipWriter,
    result::ZipResult,
    write::{ExtendedFileOptions, FileOptions, SimpleFileOptions},
};

use crate::config::{Config, ManifestJson};
use crate::curseforge::retry;

pub fn read_manifest_json(path: &Path) -> Result<ManifestJson> {
    // JSON向けにBufReaderの実装が存在するが精々600要素程度.
    // かつ、100KB程度と思われるので直に読み込む.
    let raw_data = fs::read_to_string(&path)?;
    let manifest = serde_json::from_str(&raw_data).context("failed to parse manifest json")?;
    Ok(manifest)
}

pub fn read_config(path: &Path) -> Result<Config> {
    // TOMLは小さいことがわかっているので直にやる
    let raw_data = fs::read_to_string(path)?;
    let config = toml::from_str(&raw_data).context("failed to parse config toml")?;
    Ok(config)
}

fn retryable_fetch(client: &Client, download_url: &String) -> Result<Bytes, reqwest::Error> {
    let resp = client
        .get(download_url)
        .send()
        .expect("Request Failed")
        .bytes()
        .unwrap();
    Ok(resp)
}

pub fn fetch_file(client: &Client, download_url: &String, save_path: &PathBuf) {
    if save_path.exists() {
        println!("The {:?} is already exists. skipped download.", save_path);
        return;
    }
    println!("Start down loading {} to {:?}", download_url, save_path);
    let responsed_file = retry(
        || retryable_fetch(client, download_url),
        5,
        Duration::from_secs(5),
    )
    .unwrap();

    fs::write(save_path, responsed_file).expect("Error in Writing Mods to output_folder");
    println!("Saved to {:?}", save_path);
}

pub fn copy_dir(from: &Path, to: &Path) -> Result<bool> {
    let has_skiped = true;
    let mut error = 0u16;
    fs::create_dir_all(to)?;
    if !from.is_dir() {
        eprintln!("This is not Directory, skipped: {}", from.to_string_lossy());
        return Ok(has_skiped);
    }
    if !to.is_dir() {
        eprintln!("This is not Directory, skipped: {}", from.to_string_lossy());
        return Ok(has_skiped);
    }
    fs::create_dir_all(to)?;

    WalkDir::new(from)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .for_each(|entry| {
            let from_path = entry.path();

            if from_path.is_file() {
                let rel = from_path.strip_prefix(from).unwrap();
                let to_path = Path::new(&to).join(rel);
                if let Some(parent) = to_path.parent() {
                    fs::create_dir_all(parent).expect("Error in Create directory");
                }
                println!(
                    "copy {} to {}",
                    from_path.to_string_lossy(),
                    to_path.to_string_lossy()
                );

                if let Err(e) = fs::copy(&from_path, &to_path) {
                    eprintln!("Failed to copy {:?} | {:?} :{}", from_path, to_path, e);
                }
            }
        });

    println!("Copy {:?} to {:?} is Successful.", from, to);

    Ok(!has_skiped)
}

pub fn directory_archive(directory_path: &PathBuf, archive_name: &PathBuf) -> Result<()> {
    let file = File::create(archive_name)?;
    let mut zip = ZipWriter::new(&file);
    let opts: FileOptions<ExtendedFileOptions> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    let mut buffer = Vec::new();

    // walkDirで再帰処理
    WalkDir::new(directory_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            let path = entry.path();
            let name = path.strip_prefix(directory_path).unwrap();

            if path.is_file() {
                println!("adding file {:?} as {:?}", path, name);
                zip.start_file(name.to_string_lossy(), opts.clone())
                    .unwrap();
                let mut f = File::open(path).unwrap();
                f.read_to_end(&mut buffer).unwrap();
                zip.write_all(&buffer).unwrap();
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                println!("create dir {:?}", name);
                zip.add_directory(name.to_string_lossy(), opts.clone())
                    .unwrap();
            }
        });

    zip.finish().unwrap();
    println!("Created Archive {:?}", archive_name.to_string_lossy());
    Ok(())
}
