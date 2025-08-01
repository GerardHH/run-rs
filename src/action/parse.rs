use std::{collections::BTreeMap, fs, path::PathBuf, vec};

use anyhow::Result;

use crate::parsers::parser_trait::Parser;
use crate::util::anyhow::error_with_location;

pub fn supported_extension(parsers: &Vec<Box<dyn Parser>>) -> Vec<&str> {
    parsers
        .iter()
        .flat_map(|parser| parser.supported_extension())
        .copied()
        .collect()
}

pub fn tasks(
    task_files: Vec<PathBuf>,
    parsers: &Vec<Box<dyn Parser>>,
) -> Result<BTreeMap<PathBuf, Vec<String>>> {
    let result: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();

    for task_file in task_files {
        let extension = task_file.extension();
        if extension.is_none() {
            return Err(error_with_location!(
                "Could not determine extension of '{:#?}'",
                task_file
            ));
        }
    }

    Ok(result)
}

mod tests {
    use serde::Deserialize;

    #[test]
    fn playground() {
        #[derive(Debug, Deserialize)]
        struct Config {
            name: String,
            version: String,
        }

        let json = "{ \"name\": \"test\", \"version\": 9 }";

        let config: Config = serde_json::from_str(json).unwrap();

        println!("GH: {:?}", config);

        assert_eq!(config.name, "tst");
    }
}
