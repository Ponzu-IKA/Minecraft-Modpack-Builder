use anyhow::{Context, Result};

use crate::config::ManifestJson;

pub fn read_manifestjson(raw_data: &str) -> Result<ManifestJson> {
    let manifest: ManifestJson =
        serde_json::from_str(raw_data).context("failed to parse manifest.json")?;
    Ok(manifest)
}
