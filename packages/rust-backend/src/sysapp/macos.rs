#![cfg(target_os = "macos")]
#![allow(dead_code)]
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

pub fn find_apps_macos(mut extra_dirs: Vec<String>) -> Vec<String> {
    let mut apps = vec![];
    let mut start_menu_dirs = vec![
        "/System/Applications",
        "/Applications",
        "/System/Library/PreferencePanes",
    ]
    .into_iter()
    .map(|d| String::from(d))
    .collect::<Vec<String>>();

    start_menu_dirs.append(&mut extra_dirs);

    let search_paths: Vec<PathBuf> = start_menu_dirs
        .into_iter()
        .map(|dir| PathBuf::from(dir))
        .collect::<Vec<PathBuf>>();

    for path in search_paths
        .into_iter()
        .filter(|path| fs::try_exists(path).unwrap())
    {
        for entry in WalkDir::new(path).into_iter() {
            let entry = entry.unwrap();
            let path = entry.path();
            let valid = if let Some(ext) = path.extension() {
                ext == "app" || ext == "prefPane"
            } else {
                false
            };

            if valid {
                apps.push(String::from(path.to_str().unwrap()));
            }
        }
    }
    apps
}
