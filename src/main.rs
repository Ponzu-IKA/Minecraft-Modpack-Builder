use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use clap::Parser;
use config::Args;

use crate::{
    curseforge::fetchmods,
    modloader::{VersionSet, fetch_modloader},
    utils::{copy_dir, directory_archive, read_config, read_manifest_json},
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
#[warn(unused_extern_crates)]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = read_config(Path::new("config.toml"))?;
    println!("{:?}", config);
    let manifest = read_manifest_json(Path::new(&config.manifest))?;
    let outputfolder = Path::new("./distribution");
    println!("{:?}", &manifest);
    let mut client_id_ban = config.default_config.client_id_ban;
    let mut config_ban_list = config.client_ban_list;
    client_id_ban.append(&mut config_ban_list);

    // CurseForgeからサーバーパック用のmodを取得
    // Server ID BANからクライアント系MODは除外済み
    let mods_path = {
        let mut path = PathBuf::new();
        if let Some(files) = &manifest.files {
            path = fetchmods(files, outputfolder, &config.default_config.server_id_ban)?;
        };
        path
    };

    println!("get curseforge mods is end!");

    let loader: &Vec<_> = &manifest.minecraft.mod_loaders[0].id.split('-').collect();

    let versionset = VersionSet {
        minecraft: manifest.minecraft.version.clone(),
        loader: loader[1].to_string(),
        loader_type: match loader[0] {
            "forge" => modloader::LoaderType::Forge,
            _ => modloader::LoaderType::Fabric,
        },
    };
    let loader_save_path = outputfolder.join("./loader");
    fs::create_dir_all(&loader_save_path)?;

    let pack_name = format!("{}-v{}", config.info.name, config.info.version);
    let pack_path = outputfolder.join("./exported");
    fs::create_dir_all(&pack_path)?;

    fetch_modloader(&versionset, &loader_save_path);
    'build_server_package: {
        let server_pack_path = outputfolder.join("./.server");
        copy_dir(&mods_path, &server_pack_path.join("./mods"))?;
        let override_dirs = &config.override_dirs;
        for override_dir in override_dirs {
            copy_dir(
                Path::new(&override_dir),
                &server_pack_path.join(override_dir),
            )?;
        }
        let archive_name = pack_path.join(format!("{}-server.zip", pack_name));
        println!("{}", archive_name.to_string_lossy());
        directory_archive(&server_pack_path, &archive_name)?;
    };
    'build_Client_package: {
        let client_pack_path = outputfolder.join("./.client");
        let client_overrides_path = outputfolder.join("./.client/override");

        let override_dirs = &config.override_dirs;
        for override_dir in override_dirs {
            copy_dir(
                Path::new(&override_dir),
                &client_overrides_path.join(override_dir),
            )?;
        }
        let mut manifest_json = manifest;
        manifest_json.name = config.info.name;
        manifest_json.version = config.info.version;
        manifest_json.author = config.info.author;

        let manifest_path = client_pack_path.join("./manifest.json");
        let json_str = serde_json::to_string_pretty(&manifest_json)?;
        std::fs::write(&manifest_path, json_str)?;
        let mut additional_files = config.additional_copy_files;
        additional_files.push("./modlist.html".to_string());
        for f in additional_files {
            //後でディレクトリコピーから単一ファイルコピー関数を切り出しておく
            fs::copy(Path::new(&f), outputfolder.join(&f))?;
        }

        let archive_name = pack_path.join(format!("{}-client.zip", pack_name));
        println!("{}", archive_name.to_string_lossy());
        directory_archive(&client_pack_path, &archive_name)?;
    }
    Ok(())
}
