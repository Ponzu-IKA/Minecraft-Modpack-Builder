use anyhow::Result;

use super::*;
use crate::{config::Mod, utils::read_manifestjson};

const SAMPLE_JSON: &str = r#"
    {
        "author": "TestsAreAWESOME",
        "files": [
            {
              "fileID": 5681725,
              "projectID": 908741,
              "required": true
            },
            {
              "fileID": 6420945,
              "projectID": 580555,
              "required": true
            },
            {
              "fileID": 7014291,
              "projectID": 238222,
              "required": true
            }
          ],
          "manifestType": "minecraftModpack",
          "manifestVersion": 1,
          "minecraft": {
            "modLoaders": [
              {
                "id": "forge-47.4.0",
                "primary": true
              }
            ],
            "version": "1.20.1"
          },
          "name": "CF",
        "overrides": "overrides",
      "version": "1.0.0"
    }
    "#;

#[test]
fn parse_sample() -> anyhow::Result<()> {
    let manifest = read_manifestjson(SAMPLE_JSON)?;
    assert_eq!(manifest.author, "TestsAreAWESOME");
    if let Some(files) = manifest.files {
        let file = files.get(0).unwrap();
        assert_eq!(
            file,
            &Mod {
                file_id: 5681725,
                project_id: 908741,
                required: true
            }
        );
        let file = files.get(1).unwrap();
        assert_eq!(
            file,
            &Mod {
                file_id: 6420945,
                project_id: 580555,
                required: true
            }
        );
        let file = files.get(2).unwrap();
        assert_eq!(
            file,
            &Mod {
                file_id: 7014291,
                project_id: 238222,
                required: true
            }
        );
    };
    assert_eq!(manifest.version, "1.0.0");

    Ok(())
}
