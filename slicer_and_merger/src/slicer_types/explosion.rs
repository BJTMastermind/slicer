use image::{RgbaImage, GenericImage};
use std::fs;
use crate::util::hightlight_image;

pub fn slice_explosion(from_path: &String, to_path: &String, leftover_path: &String) {
    let textures = vec![
        ("explosion_0", 0, 0),
        ("explosion_1", 32, 0),
        ("explosion_2", 64, 0),
        ("explosion_3", 96, 0),
        ("explosion_4", 0, 32),
        ("explosion_5", 32, 32),
        ("explosion_6", 64, 32),
        ("explosion_7", 96, 32),
        ("explosion_8", 0, 64),
        ("explosion_9", 32, 64),
        ("explosion_10", 64, 64),
        ("explosion_11", 96, 64),
        ("explosion_12", 0, 96),
        ("explosion_13", 32, 96),
        ("explosion_14", 64, 96),
        ("explosion_15", 96, 96)];
    if leftover_path != &"".to_string() {
        let used_path = format!("{}{}", leftover_path, "assets/minecraft/textures/entity/explosion.png");
        fs::copy(format!("{}{}", from_path, "assets/minecraft/textures/entity/explosion.png"), used_path).unwrap();
    }
    for i in 0..textures.len() {
        get_image(&from_path, &to_path, &leftover_path, textures[i].0, textures[i].1, textures[i].2)
    }
}

fn get_image(from_path: &String, to_path: &String, leftover_path: &String, name: &str, x: u32, y: u32) {
    let mut base: RgbaImage = image::open(format!("{}{}", from_path, "assets/minecraft/textures/entity/explosion.png")).unwrap().to_rgba8();
    let mut mark: RgbaImage;
    let out_path = format!("{}{}{}{}", to_path, "assets/minecraft/textures/particle/", &name, ".png");
    let texture: RgbaImage = base.sub_image(x, y, 32, 32).to_image();
    texture.save(&out_path).unwrap();
    if leftover_path != &"".to_string() {
        mark = image::open(format!("{}{}", leftover_path, "assets/minecraft/textures/entity/explosion.png")).unwrap().to_rgba8();
        hightlight_image(&mut mark, format!("{}{}", leftover_path, "assets/minecraft/textures/entity/explosion.png"), x, y, 32, 32);
    }
    println!("{}", out_path);
}