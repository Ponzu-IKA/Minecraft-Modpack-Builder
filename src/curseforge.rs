use crate::{config::Mod, utils::fetch_file};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;
use serde_derive::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    hash::RandomState,
    path::{Path, PathBuf},
    sync::Mutex,
};

#[derive(Deserialize, Debug)]
struct FileResponse {
    data: FileData,
}

#[derive(Deserialize, Debug)]
#[allow(warnings)]
struct FileData {
    fileName: String,
    downloadUrl: String,
}

pub fn fetchmods(
    cf_apikey: &String,
    mod_list: &Vec<Mod>,
    output_folder: &Path,
    server_banned_mods: &Vec<u32>,
) -> anyhow::Result<PathBuf> {
    let output_folder = output_folder.join("mods");
    fs::create_dir_all(&output_folder)?;

    let client = Client::new();

    mod_list.par_iter().for_each(|cf_mod| {
        if server_banned_mods
            .into_iter()
            .find(|&&id| id == cf_mod.project_id)
            .is_some()
        {
            println!("skip detected client mod: (id: {})", cf_mod.project_id);
            return;
        }
        println!(
            "Downloading projectID={} fileID={}",
            cf_mod.project_id, cf_mod.file_id
        );
        let url = format!(
            "https://api.curseforge.com/v1/mods/{}/files/{}",
            cf_mod.project_id, cf_mod.file_id
        );

        let response: FileResponse = client
            .get(&url)
            .header("x-api-key", cf_apikey)
            .send()
            .expect("Request Failed")
            .json()
            .expect("Couldn't get JSON.");

        println!("JSON: {:?}", response);
        // JSONからファイル名を確保
        let file_path = output_folder.join(&response.data.fileName);

        // JSONから有効なダウンロードURLを確保.
        let download_url = response.data.downloadUrl;

        // だうんろーど.
        fetch_file(&client, &download_url, &file_path);
    });
    Ok(output_folder)
}
