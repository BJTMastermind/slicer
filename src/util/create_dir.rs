use std::{fs, path::Path};

pub enum PackDirType {
    GUI,
    NONE,
}

pub fn create_texture_pack_dir(out_dir: &str, pack_dir_type: &PackDirType) {
    use PackDirType::*;
    let paths = &format!("{}{}", out_dir, "assets/minecraft/textures/");
    if !Path::new(&paths).exists() {
        fs::create_dir_all(&paths).expect("Failed to create directory!");
    }
    let texture_folders = match pack_dir_type {
        GUI => create_texture_pack_path_gui(paths),
        NONE => create_texture_pack_path(paths),
    };
    create_dirs(&texture_folders);
}

fn create_texture_pack_path_gui(paths: &str) -> Vec<String> {
    vec![
        format!("{}{}", paths, "painting/"),
        format!("{}{}", paths, "particle/"),
        format!("{}{}", paths, "entity/"),
        format!("{}{}", paths, "gui/"),
        format!("{}{}", paths, "gui/container/"),
    ]
}

fn create_texture_pack_path(paths: &str) -> Vec<String> {
    vec![
        format!("{}{}", paths, "painting/"),
        format!("{}{}", paths, "mob_effect/"),
        format!("{}{}", paths, "particle/"),
        format!("{}{}", paths, "entity/"),
    ]
}

fn create_dirs(folders: &[String]) {
    for folder in folders.iter() {
        if !Path::new(&folder).exists() {
            fs::create_dir(&folder).expect("Failed to create directory!");
        }
    }
}
