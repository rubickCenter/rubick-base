use serde::{Deserialize, Serialize};
mod linux;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub icon_path: Option<Vec<String>>,
    pub description: String,
    pub command: String,
    pub desktop_entry_path: Option<String>,
}

#[allow(dead_code)]
pub fn find_apps() -> String {
    let apps = linux::find_apps_linux();
    serde_json::to_string(&apps).unwrap()
}
