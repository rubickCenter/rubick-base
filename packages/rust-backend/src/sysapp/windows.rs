use std::{env, path::PathBuf};

fn main() {
    let start_menu_dirs = vec![
        format!(
            r"{}\Microsoft\Windows\Start Menu\Programs",
            env::var("ProgramData").unwrap()
        ),
        format!(
            r"{}\Microsoft\Windows\Start Menu\Programs",
            env::var("AppData").unwrap()
        ),
        format!(r"{}\OneDrive\Desktop", env::var("USERPROFILE").unwrap()),
        format!(r"{}\Desktop", env::var("PUBLIC").unwrap()),
    ];

    let mut search_paths: Vec<PathBuf> = env::split_paths(&env::var("PATH").unwrap()).collect();
    search_paths.append(&mut start_menu_dirs.into_iter().map(|dir| PathBuf::from(dir)).collect::<Vec<PathBuf>>());

    println!(
        "{:?}",
        search_paths
    );
}
