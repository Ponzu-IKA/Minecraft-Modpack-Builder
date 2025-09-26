use crate::{config::Mod, utils::fetch_file};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;
use serde_derive::Deserialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    thread,
    time::Duration,
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

pub fn retry<F, T, E>(mut f: F, retries: usize, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Debug,
{
    for attempt in 1..=retries {
        match f() {
            Ok(val) => return Ok(val),
            Err(e) => {
                eprintln!("Failed ({} times): {:?}, retrying...", attempt, e);
                thread::sleep(delay);
            }
        }
    }
    f()
}

fn get_json(client: &Client, url: &String) -> Result<FileResponse, reqwest::Error> {
    println!("{}", url);
    client.get(url).send().expect("Request Failed").json()
}
fn get_file() {}

pub fn fetchmods(
    mod_list: &Vec<Mod>,
    output_folder: &Path,
    server_banned_mods: &Vec<u32>,
) -> anyhow::Result<PathBuf> {
    let sleep = Duration::from_secs(5); //APIがパンクしちゃうのでちょっと長めに待たせる

    let output_folder = output_folder.join("mods");
    fs::create_dir_all(&output_folder)?;

    let client = Client::new();

    mod_list.par_iter().for_each(|cf_mod| {
        if server_banned_mods
            .iter()
            .any(|&id| id == cf_mod.project_id)
        {
            println!("skip detected client mod: (id: {})", cf_mod.project_id);
            return;
        }
        println!(
            "Downloading projectID={} fileID={}",
            cf_mod.project_id, cf_mod.file_id
        );
        let url = format!(
            "https://api.curse.tools/v1/cf/mods/{}/files/{}",
            cf_mod.project_id, cf_mod.file_id
        );
        let response = retry(|| get_json(&client, &url), 5, sleep).unwrap();

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
