use std::{fs, path::Path};

pub fn create_dirs(out_dir: &String, make_gui: bool) {
    let paths = format!("{}{}", out_dir, "assets/minecraft/textures/");
    if !Path::new(&paths).exists() {
        fs::create_dir_all(&paths).expect("Could not create directory!");
    }
    
    let texture_folders: Vec<String>;
    if make_gui {
        texture_folders = vec![format!("{}{}", paths, "painting/"), format!("{}{}", paths, "particle/"),
        format!("{}{}", paths, "entity/"), format!("{}{}", paths, "gui/"), format!("{}{}", paths, "gui/container/")];
    } else {
        texture_folders = vec![format!("{}{}", paths, "painting/"), format!("{}{}", paths, "mob_effect/"),
        format!("{}{}", paths, "particle/"), format!("{}{}", paths, "entity/")];
    }

    for i in 0..texture_folders.len() {
        if !Path::new(&texture_folders[i]).exists() {
            fs::create_dir(&texture_folders[i]).expect("Could not create directory!");
        }
    }
}