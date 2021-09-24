#![allow(dead_code)]

// MIT https://github.com/Psykopear/fuzzle
use crate::sysapp::SearchResult;
use ini::Ini;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use xdg::BaseDirectories;

struct AppParser {
    icon_map: HashMap<String, Vec<String>>,
    lang: Option<String>,
}

fn get_appdirs() -> Vec<PathBuf> {
    // get appdirs
    let base_dirs = BaseDirectories::new()
        .expect("Can't find xdg directories! Good luck and thanks for all the fish");
    let mut app_dirs: Vec<PathBuf> = Vec::new();
    app_dirs.push(base_dirs.get_data_home());
    app_dirs.append(&mut base_dirs.get_data_dirs());
    app_dirs
}

impl AppParser {
    fn new_parser() -> AppParser {
        // Get icon dirs, add "icons" (/usr/share/icons ecc...)
        let mut icon_dirs: Vec<String> = get_appdirs()
            .iter()
            .map(|dd| dd.join("icons").to_str().unwrap().to_string())
            .collect();
        // Add $HOME/.icons
        if let Ok(home) = env::var("HOME") {
            icon_dirs.insert(0, format!("{}/.icons", home));
        }
        icon_dirs.insert(0, "/usr/share/pixmaps".to_string());

        let mut icon_map: HashMap<String, Vec<String>> = HashMap::new();

        for dir in icon_dirs
            .into_iter()
            .filter(|path| fs::try_exists(path).unwrap())
        {
            for entry in WalkDir::new(dir).into_iter() {
                let entry = entry.unwrap();
                let icon_path = entry.path();
                let valid = if let Some(ext) = icon_path.extension() {
                    ext == "png" || ext == "svg" || ext == "xpm"
                } else {
                    false
                };
                if valid {
                    let file_path = String::from(icon_path.to_str().unwrap());
                    let file_name = String::from(icon_path.file_name().unwrap().to_str().unwrap());
                    let file_name = file_name.replace(
                        format!(".{}", icon_path.extension().unwrap().to_str().unwrap()).as_str(),
                        "",
                    );

                    match icon_map.get(&file_name) {
                        Some(v) => {
                            // todo: update vec directly
                            let mut nv = v.clone();
                            nv.push(file_path);
                            icon_map.remove(&file_name);
                            icon_map.insert(file_name, nv);
                        }
                        None => {
                            icon_map.insert(file_name, vec![file_path]);
                        }
                    };
                }
            }
        }

        let lang = match env::var("LANG") {
            Ok(lang) => {
                if let Some(lang) = lang.split(".").next() {
                    Some(lang.to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        };

        AppParser { icon_map, lang }
    }

    /// Given an icon name, search for the icon file.
    fn search_icon(&self, icon: &str) -> Option<Vec<String>> {
        // if icon is a path return it
        if fs::try_exists(icon).unwrap() {
            return Some(vec![icon.to_string()]);
        }

        if let Some(icon) = self.icon_map.get(&icon.to_string()) {
            return Some(icon.to_vec());
        }

        None
    }

    /// Given a desktop file path, try to build a SearchResult
    fn searchresult_from_desktopentry(&self, desktop_file_path: &Path) -> Option<SearchResult> {
        let suffix;

        if let Some(lang) = &self.lang {
            suffix = format!("[{}]", lang);
        } else {
            suffix = "".to_string();
        }

        let name = format!("Name{}", suffix);
        let comment = format!("Comment{}", suffix);
        // If anything we need can't be found, return None
        let info = match Ini::load_from_file(&desktop_file_path) {
            Ok(info) => info,
            Err(_) => return None,
        };
        let section = match info.section(Some("Desktop Entry")) {
            Some(sec) => sec,
            None => return None,
        };
        let name = match section.get(name) {
            Some(name) => name.to_string(),
            None => match section.get("Name") {
                Some(name) => name.to_string(),
                None => return None,
            },
        };
        let description = match section.get(comment) {
            Some(description) => description.to_string(),
            None => match section.get("Comment") {
                Some(description) => description.to_string(),
                None => return None,
            },
        };
        let icon = match section.get("Icon") {
            Some(icon) => icon,
            None => return None,
        };
        let command = match section.get("Exec") {
            Some(command) => command.to_string(),
            None => return None,
        };

        let desktop_entry_path = match desktop_file_path.to_str() {
            Some(path) => Some(path.to_string()),
            None => return None,
        };

        Some(SearchResult {
            icon_path: self.search_icon(icon),
            desktop_entry_path,
            name,
            description,
            command,
        })
    }
}

/// Given a binary file path, try to build a SearchResult
// fn searchresult_from_bin(command_path: &Path) -> Option<SearchResult> {
//     let name = match command_path.file_stem() {
//         Some(os_str) => {
//             if let Some(str_ref) = os_str.to_str() {
//                 str_ref.to_string()
//             } else {
//                 return None;
//             }
//         }
//         None => return None,
//     };

//     let description = match command_path.as_os_str().to_str() {
//         Some(desc) => desc.to_string(),
//         None => return None,
//     };
//     let command = description.clone();

//     Some(SearchResult {
//         icon_path: search_icon("terminal"),
//         desktop_entry_path: None,
//         name,
//         description,
//         command,
//     })
// }

/// Search all applications and collect them in a Vec of SearchResult
/// This should be the only public api in this module.
pub fn find_apps_linux() -> Vec<SearchResult> {
    let app_parser = AppParser::new_parser();
    let mut results: Vec<SearchResult> = Vec::new();

    // Build SearchResults for all desktop files we can find
    for mut data_dir in get_appdirs() {
        data_dir.push("applications");
        if let Ok(data_dir) = fs::read_dir(data_dir) {
            results.append(
                &mut data_dir
                    .filter_map(|path| {
                        app_parser.searchresult_from_desktopentry(&path.unwrap().path())
                    })
                    .collect(),
            );
        }
    }

    // Now build SearchResults for all binaries we can find
    // let key = "PATH";
    // match env::var_os(key) {
    //     Some(paths) => {
    //         for path in env::split_paths(&paths) {
    //             if let Ok(entries) = fs::read_dir(path) {
    //                 results.append(
    //                     &mut entries
    //                         .filter_map(|path| searchresult_from_bin(&path.unwrap().path()))
    //                         .collect(),
    //                 );
    //             }
    //         }
    //     }
    //     None => println!("{} is not defined in the environment.", key),
    // }
    // That's it, return
    results
}
