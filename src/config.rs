use clap::Parser;
use serde_derive::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about = "Minecraft Pack Builder")]
pub struct Args {}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub info: Info,
    // 取捨択一,
    // IDで指定したいか、ダウンロード後のmodで指定したいかみたいな
    pub clientside_string: Option<Vec<String>>,
    pub clientside_id: Option<Vec<u32>>,
    pub serverside_string: Option<Vec<String>>,
    pub serverside_id: Option<Vec<String>>,
    pub changed_configs: Option<Vec<String>>,
}

// manifest.jsonを上書きするためのinfo
#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub author: String,
}

// manifest.jsonを読み取るためのstruct
#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestJson {
    pub author: String,
    pub version: String,
    pub name: String,
    pub files: Option<Vec<Mod>>,

    //ここから下に変更は来ないはず...?
    minecraft: Minecraft,
    #[serde(rename = "manifestType")]
    manifest_type: String,
    #[serde(rename = "manifestVersion")]
    manifest_version: u8,
    overrides: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Minecraft {
    version: String,
    #[serde(rename = "modLoaders")]
    mod_loaders: Option<Vec<ModLoader>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ModLoader {
    id: String,
    primary: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Mod {
    #[serde(rename = "fileID")]
    pub file_id: u32,
    #[serde(rename = "projectID")]
    pub project_id: u32,
    pub required: bool,
}
