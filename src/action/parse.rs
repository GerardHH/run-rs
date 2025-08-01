use std::{collections::BTreeMap, path::PathBuf, vec};

use anyhow::Result;

pub fn tasks(task_files: Vec<PathBuf>) -> Result<BTreeMap<PathBuf, Vec<String>>> {
    let result: BTreeMap<PathBuf, Vec<String>> = task_files
        .into_iter()
        .map(|path| (path, vec!["task".to_string()]))
        .collect();

    Ok(result)
}
