use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use reqwest::blocking::Client;

use crate::config::{Config, ManifestJson};

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

pub fn fetch_file(client: &Client, download_url: &String, save_path: &PathBuf) {
    if save_path.exists() {
        println!("The {:?} is already exists. skipped download.", save_path);
        return;
    }
    println!("Start down loading {} to {:?}", download_url, save_path);
    let responsed_file = client
        .get(download_url)
        .send()
        .expect("Request Failed")
        .bytes()
        .expect("Error in read Bytes.");

    fs::write(save_path, responsed_file).expect("Error in Writing Mods to output_folder");
    println!("Saved to {:?}", save_path);
}
