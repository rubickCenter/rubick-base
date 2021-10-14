use serde::{Deserialize, Serialize};
mod linux;
mod macos;
mod windows;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub icon_path: Option<Vec<String>>,
    pub description: String,
    pub command: String,
    pub desktop_entry_path: Option<String>,
}

// only linux can returns detail info now, parsing windows/macos shortcut app info is still need to be done
#[allow(dead_code)]
pub fn find_apps(_detail_json: bool, extra_dirs: Option<Vec<String>>) -> Vec<String> {
    let extra_dirs = match extra_dirs {
        Some(dir) => dir,
        None => vec![],
    };

    #[cfg(target_os = "linux")]
    let apps = linux::find_apps_linux(_detail_json, extra_dirs);

    #[cfg(target_os = "windows")]
    let apps = windows::find_apps_windows(extra_dirs);

    #[cfg(target_os = "macos")]
    let apps = macos::find_apps_macos(extra_dirs);

    apps
}
