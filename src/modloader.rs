use std::path::PathBuf;

use reqwest::blocking::Client;

use crate::utils::fetch_file;

pub enum LoaderType {
    Forge,
    NeoForge,
    Fabric,
}

impl LoaderType {
    fn get_name(&self) -> &'static str {
        match self {
            LoaderType::Forge => "forge",
            LoaderType::NeoForge => "neoforge",
            LoaderType::Fabric => "fabric",
        }
    }
}

impl VersionSet {
    fn url(&self) -> String {
        match self.loader_type {
            LoaderType::Forge => {
                if self.minecraft == "1.7.10" {
                    format!(
                        "https://maven.minecraftforge.net/net/minecraftforge/forge/{mc_version}-{loader_version}-{mc_version}/forge-{mc_version}-{loader_version}-{mc_version}-installer.jar",
                        mc_version = self.minecraft,
                        loader_version = self.loader
                    )
                } else {
                    format!(
                        "https://maven.minecraftforge.net/net/minecraftforge/forge/{mc_version}-{loader_version}/forge-{mc_version}-{loader_version}-installer.jar",
                        mc_version = self.minecraft,
                        loader_version = self.loader
                    )
                }
            }
            LoaderType::NeoForge => {
                format!(
                    "https://maven.neoforged.net/releases/net/neoforged/neoforge/{mc_ver}.167/neoforge-{loader_version}-installer.jar",
                    mc_ver = self
                        .loader
                        .get(9..self.loader.len() - 1)
                        .expect("Format Error!"), // neoforge. で9文字.
                    loader_version = self.loader
                )
            }
            LoaderType::Fabric => String::new(),
        }
    }
}

pub struct VersionSet {
    pub minecraft: String,
    pub loader: String,
    pub loader_type: LoaderType,
}

pub fn fetch_modloader(version_set: &VersionSet, path: &PathBuf) {
    let client = Client::new();
    fetch_file(
        &client,
        &version_set.url(),
        &path.join(format!(
            "{ltype}-{lver}-{mcver}-server_installer.jar",
            ltype = version_set.loader_type.get_name(),
            lver = version_set.loader,
            mcver = version_set.minecraft
        )),
    );
}
