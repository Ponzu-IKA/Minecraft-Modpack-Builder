use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use config::Args;

use crate::{
    curseforge::fetchmods,
    modloader::{VersionSet, fetch_modloader},
    utils::{read_config, read_manifest_json},
};

mod build;
mod config;
mod curseforge;
mod modloader;
mod utils;
/*
#[cfg(test)]
mod test;
*/

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = read_config(Path::new("config.toml"))?;
    println!("{:?}", config);
    let manifest = read_manifest_json(Path::new(&config.manifest))?;
    let outputfolder = Path::new("./distribution");
    println!("{:?}", manifest);
    let mut client_id_ban = config.default_config.client_id_ban;
    let mut config_ban_list = config.client_ban_list;
    client_id_ban.append(&mut config_ban_list);

    // CurseForgeからサーバーパック用のmodを取得
    // Server ID BANからクライアント系MODは除外済み
    let mods_path = {
        let mut path = PathBuf::new();
        if let Some(files) = &manifest.files {
            path = fetchmods(
                &config.curseforge_api_key,
                files,
                &outputfolder,
                &config.default_config.server_id_ban,
            )?;
        };
        path
    };
    let loader: Vec<_> = manifest.minecraft.mod_loaders[0].id.split('-').collect();

    let versionset = VersionSet {
        minecraft: manifest.minecraft.version,
        loader: loader[1].to_string(),
        loader_type: match loader[0] {
            "forge" => modloader::LoaderType::Forge,
            _ => modloader::LoaderType::Fabric,
        },
    };
    let loader_save_path = outputfolder.join("./loader/");
    fs::create_dir_all(&loader_save_path)?;

    fetch_modloader(&versionset, &loader_save_path);
    'build_server_package: {};
    let server_pack_path = outputfolder.join("./.server");

    println!("Hello, world!");

    Ok(())
}
