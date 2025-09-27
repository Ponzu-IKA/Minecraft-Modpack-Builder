use clap::Parser;
use serde_derive::{Deserialize, Serialize};

const DESC: &str = r#"Minecraft Modpack Builder       
Licenced by: MIT-License (c) 2025 Ponzu-IKA(TsukamattaHiyoko)"#;

#[derive(Parser, Debug)]
#[command(author, version, about = DESC)]
pub struct Args {}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub info: Info,
    #[serde(default = "default_manifest")]
    pub manifest: String,
    #[serde(default)]
    pub default_config: DefaultConfig,

    #[serde(default = "default_dirs")]
    pub override_dirs: Vec<String>,

    // modlist.htmlのように一切の変更なしにclientパックにぶち込むやつはここで定義
    // 例:  |overrides
    //      |- modlist.html
    //      |- LICENCE.md
    #[serde(default)]
    pub additional_copy_files: Vec<String>,
    
    /// ProjectIDを指定することでサーバーパックにクライアントMODが入ることを阻止できる.
    /// デフォルトの指定じゃ足りないときに使うよ.
    #[serde(default)]
    pub additional_noneeds_with_server: Vec<u32>,
}

fn default_dirs() -> Vec<String> {
    vec!["./config".to_string(), "./kubejs".to_string()]
}

fn default_manifest() -> String {
    "./manifest.json".to_string()
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub no_needs_with_server: Vec<u32>,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            no_needs_with_server: default_no_needs_with_server(),
        }
    }
}

fn default_no_needs_with_server() -> Vec<u32> {
    // サーバーにいらないmod.
    vec![
        908741, // Embeddium
        352491, // ModernUI
        250398, // Controlling
        60089,  // MouseTweaks
    ]
}

// manifest.jsonを上書きするためのinfo
#[derive(Debug, Deserialize, Default)]
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

    // ModLoaderを取得するために使うよ
    pub minecraft: Minecraft,

    // 上書きしても情報が残るようにする
    #[serde(rename = "manifestType")]
    manifest_type: String,
    #[serde(rename = "manifestVersion")]
    manifest_version: u8,
    overrides: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Minecraft {
    #[serde(default, rename = "modLoaders")]
    pub mod_loaders: Vec<ModLoader>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModLoader {
    pub id: String,
    pub primary: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Mod {
    #[serde(rename = "fileID")]
    pub file_id: u32,
    #[serde(rename = "projectID")]
    pub project_id: u32,
    pub required: bool,
}
