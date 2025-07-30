use std::env::{self, VarError};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::util::anyhow::{error_with_location, with_location, with_location_msg};

/// Find all supported task files and return them.
///
/// Supported files:
/// * json (specifically VSCode task file format)
///
/// This function does not validate the files contents.
///
/// # Developer note
///
/// This function is hard to test because of `std::env` usage. Leave as much logic out of this
/// function as possible.
///
/// # Return
///
/// * Vector of full paths to task files.
pub fn task_files() -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];

    let config_dir = config_dir(
        env::var("RUN_RS_CONFIG"),
        env::var("XDG_CONFIG_HOME"),
        env::var("HOME"),
    )?;
    result.append(&mut gather_task_files(&config_dir));

    let cwd = with_location!(env::current_dir())?;
    let path = vscode_tasks_file(cwd)?;
    result.push(path);

    Ok(result)
}

/// Determines the directory where the configurations will be stored based on passed environment
/// variables in this order:
/// 1. `run_rs_config`
/// 2. `xdg_config_home`
/// 3. `home`
///
/// # Arguments
///
/// * run_rs_config - `std::env::var("RUN_RS_CONFIG")`
/// * xdg_config_home - `std::env::var("XDG_CONFIG_HOME")`
/// * home - `std::env::var("home")`
///
/// # Returns
///
/// * Ok(`run_rs_config`), or if empty;
/// * Ok(`xdg_config_home`/run_rs), or if empty;
/// * Ok(`home`/.config/run_rs), or if empty;
/// * Err.
fn config_dir(
    run_rs_config: Result<String, VarError>,
    xdg_config_home: Result<String, VarError>,
    home: Result<String, VarError>,
) -> Result<PathBuf> {
    if let Ok(path) = run_rs_config {
        return Ok(PathBuf::from(path));
    }

    if let Ok(path) = xdg_config_home {
        return Ok(PathBuf::from(format!("{path}/run_rs")));
    }

    let path = with_location_msg!(
        home,
        "'RUN_RS_CONFIG', 'XDG_CONFIG_HOME' and 'HOME' environment variable not set"
    )?;
    Ok(PathBuf::from(format!("{path}/.config/run_rs")))
}

/// Walk `dir` and find all files with supported extension.
///
/// Supported extensions:
/// * json (should be VSCode task format)
///
/// This function does not check the contents of the files, only finds their location.
///
/// # Arguments
///
/// * dir - The location on file system to traverse.
///
/// # Return
///
/// * Vector of full path to found files.
fn gather_task_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = vec![];

    for entry in WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(ext) = entry.path().extension() {
            if ext == "json" {
                result.push(entry.into_path());
            }
        }
    }

    result
}

/// Determines the full path of the VSCode tasks file `tasks.json` from current working directory.
/// Supports both `tasks.json` and `.vscode/tasks.json` in cwd.
///
/// # Arguments
///
/// * cwd - `std::env::current_dir()`
///
/// # Returns
///
/// * Full path to either `tasks.json` or `.vscode/tasks.json`.
/// * Err if not found.
fn vscode_tasks_file(cwd: PathBuf) -> Result<PathBuf> {
    for entry in WalkDir::new(cwd.clone())
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        match entry
            .file_name()
            .to_str()
            .context("Could not extract file name whilst searching for `tasks.json`")?
        {
            "tasks.json" => return Ok(entry.into_path()),
            ".vscode" => {
                if entry.file_type().is_dir() {
                    return vscode_tasks_file(cwd.join(".vscode"));
                }
                return Err(error_with_location!(
                    "Could not find `tasks.json` or `.vscode` in `{}`",
                    cwd.to_str().unwrap_or("cwd")
                ));
            }
            _ => {}
        };
    }

    Err(error_with_location!(
        "Could not find `tasks.json` or `.vscode/tasks.json` in `{}`",
        cwd.to_str().unwrap_or("cwd")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    #[macro_export]
    macro_rules! assert_unordered_eq {
        ($a:expr, $b:expr $(,)?) => {{
            let mut a = $a.clone();
            let mut b = $b.clone();
            a.sort();
            b.sort();
            assert_eq!(a, b, "Vectors are not equal ignoring order");
        }};
    }

    #[test]
    fn config_dir_no_env() {
        let run_rs_config: Result<String, VarError> = Err(VarError::NotPresent);
        let xdg_config_home: Result<String, VarError> = Err(VarError::NotPresent);
        let home: Result<String, VarError> = Err(VarError::NotPresent);

        assert!(config_dir(run_rs_config, xdg_config_home, home).is_err());
    }

    #[test]
    fn config_dir_run_rs_config() {
        let run_rs_config: Result<String, VarError> = Ok("RUN_RS_CONFIG".to_string());
        let xdg_config_home: Result<String, VarError> = Ok("XDG_CONFIG_HOME".to_string());
        let home: Result<String, VarError> = Ok("HOME".to_string());

        assert_eq!(
            config_dir(run_rs_config, xdg_config_home, home).unwrap(),
            PathBuf::from("RUN_RS_CONFIG")
        );
    }

    #[test]
    fn config_dir_xdg_config_home() {
        let run_rs_config: Result<String, VarError> = Err(VarError::NotPresent);
        let xdg_config_home: Result<String, VarError> = Ok("XDG_CONFIG_HOME".to_string());
        let home: Result<String, VarError> = Ok("HOME".to_string());

        assert_eq!(
            config_dir(run_rs_config, xdg_config_home, home).unwrap(),
            PathBuf::from("XDG_CONFIG_HOME/run_rs")
        );
    }

    #[test]
    fn config_dir_home() {
        let run_rs_config: Result<String, VarError> = Err(VarError::NotPresent);
        let xdg_config_home: Result<String, VarError> = Err(VarError::NotPresent);
        let home: Result<String, VarError> = Ok("HOME".to_string());

        assert_eq!(
            config_dir(run_rs_config, xdg_config_home, home).unwrap(),
            PathBuf::from("HOME/.config/run_rs")
        );
    }

    #[test]
    fn gather_task_files_no_files() {
        let temp_fs = assert_fs::TempDir::new().unwrap();

        let work_dir = temp_fs.to_path_buf();

        assert!(gather_task_files(&work_dir).is_empty());
    }

    #[test]
    fn gather_task_files_json_in_tree() {
        let temp_fs = TempDir::new().unwrap();

        temp_fs.child("1.json").touch().unwrap();

        let dir_one = temp_fs.child("dir_one");
        dir_one.child("2.json").touch().unwrap();

        let dir_two = dir_one.child("dir_two");
        dir_two.child("3.json").touch().unwrap();

        let dir_three = temp_fs.child("dir_three");
        dir_three.child("4.json").touch().unwrap();

        let work_dir = temp_fs.to_path_buf();

        assert_unordered_eq!(
            gather_task_files(&work_dir),
            vec![
                work_dir.join("1.json"),
                work_dir.join("dir_one/2.json"),
                work_dir.join("dir_one/dir_two/3.json"),
                work_dir.join("dir_three/4.json")
            ]
        );
    }

    #[test]
    fn vscode_tasks_no_tasks_json_in_cwd() {
        let temp_fs = TempDir::new().unwrap();

        let cwd = temp_fs.path().to_path_buf();

        let result = vscode_tasks_file(cwd.clone());

        assert!(result.is_err());
    }

    #[test]
    fn vscode_tasks_tasks_json_in_cwd() {
        let temp_fs = TempDir::new().unwrap();
        temp_fs.child("tasks.json").touch().unwrap();

        let cwd = temp_fs.path().to_path_buf();

        let result = vscode_tasks_file(cwd.clone()).unwrap();

        assert_eq!(result, cwd.join("tasks.json"));
    }

    #[test]
    fn vscode_tasks_tasks_json_in_vscode_in_cwd() {
        let temp_fs = TempDir::new().unwrap();
        temp_fs
            .child(".vscode")
            .child("tasks.json")
            .touch()
            .unwrap();

        let cwd = temp_fs.path().to_path_buf();

        let result = vscode_tasks_file(cwd.clone()).unwrap();

        assert_eq!(result, cwd.join(".vscode/tasks.json"));
    }

    #[test]
    fn vscode_tasks_no_tasks_json_in_vscode_in_cwd() {
        let temp_fs = TempDir::new().unwrap();
        temp_fs.child(".vscode").touch().unwrap();

        let cwd = temp_fs.path().to_path_buf();

        let result = vscode_tasks_file(cwd.clone());

        assert!(result.is_err());
    }
}
