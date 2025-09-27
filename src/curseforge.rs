use crate::{
    config::Mod,
    logger::{error, info, warn},
    utils::{DownloadError, fetch_file},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;
use serde_derive::Deserialize;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    sync::Mutex,
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
                error(format!("Failed ({} times): {:?}, retrying...", attempt, e));
                thread::sleep(delay);
            }
        }
    }
    f()
}

fn get_json(client: &Client, url: &String) -> Result<FileResponse, reqwest::Error> {
    info(url);
    client.get(url).send().expect("Request Failed").json()
}

pub fn fetchmods(
    mod_list: &Vec<Mod>,
    output_folder: &Path,
    server_banned_mods: &[u32],
) -> anyhow::Result<PathBuf> {
    let sleep = Duration::from_secs(5); //APIがパンクしちゃうのでちょっと長めに待たせる

    let output_folder = output_folder.join("mods");
    fs::create_dir_all(&output_folder)?;
    let modcount = mod_list.len();
    let downloading_count = Mutex::new(0u16);
    let downloaded_count = Mutex::new(0u16);
    let client = Client::new();

    mod_list.par_iter().for_each(|cf_mod| {
        if server_banned_mods.contains(&cf_mod.project_id) {
            warn(format!(
                "skip detected client mod: (id: {})",
                cf_mod.project_id
            ));
        } else {
            info(format!(
                "Downloading({:<03}/{:<03}) projectID={:<8} fileID={:<8}",
                {
                    let mut num = downloading_count.lock().unwrap();
                    *num += 1;
                    *num
                },
                modcount,
                cf_mod.project_id,
                cf_mod.file_id
            ));
            let url = format!(
                "https://api.curse.tools/v1/cf/mods/{}/files/{}",
                cf_mod.project_id, cf_mod.file_id
            );
            let response = retry(|| get_json(&client, &url), 5, sleep).unwrap();

            // JSONからファイル名を確保
            let file_name = response.data.fileName;
            let file_path = output_folder.join(&file_name);

            // JSONから有効なダウンロードURLを確保.
            let download_url = response.data.downloadUrl;

            // だうんろーど.
            match fetch_file(&client, &download_url, &file_path) {
                Ok(()) => info(format!(
                    "Downloaded ({:<03}/{:<03}){}",
                    {
                        let mut num = downloaded_count.lock().unwrap();
                        *num += 1;
                        *num
                    },
                    modcount,
                    &file_name
                )),
                Err(DownloadError::Skipped) => warn(format!("{} has been skiped", &file_name)),
                Err(e) => warn(format!("{} was not downloded: {:?}", &file_name, e)),
            };
        };
    });
    Ok(output_folder)
}
