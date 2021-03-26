use image::{RgbaImage, GenericImage};
use std::fs;
use crate::util::hightlight_image;

pub fn slice_sweep(from_path: &String, to_path: &String, leftover_path: &String) {
    let textures = vec![
        ("sweep_0", 0, 0),
        ("sweep_1", 32, 0),
        ("sweep_2", 64, 0),
        ("sweep_3", 96, 0),
        ("sweep_4", 0, 16),
        ("sweep_5", 32, 16),
        ("sweep_6", 64, 16),
        ("sweep_7", 96, 16)];
    if leftover_path != &"".to_string() {
        let used_path = format!("{}{}", leftover_path, "assets/minecraft/textures/entity/sweep.png");
        fs::copy(format!("{}{}", from_path, "assets/minecraft/textures/entity/sweep.png"), used_path).unwrap();
    }
    for i in 0..textures.len() {
        get_image(&from_path, &to_path, &leftover_path, textures[i].0, textures[i].1, textures[i].2)
    }
}

fn get_image(from_path: &String, to_path: &String, leftover_path: &String, name: &str, x: u32, y: u32) {
    let mut base: RgbaImage = image::open(format!("{}{}", from_path, "assets/minecraft/textures/entity/sweep.png")).unwrap().to_rgba8();
    let mut mark: RgbaImage;
    let out_path = format!("{}{}{}{}", to_path, "assets/minecraft/textures/particle/", &name, ".png");
    let texture: RgbaImage = base.sub_image(x, y, 32, 16).to_image();
    texture.save(&out_path).unwrap();
    if leftover_path != &"".to_string() {
        mark = image::open(format!("{}{}", leftover_path, "assets/minecraft/textures/entity/sweep.png")).unwrap().to_rgba8();
        hightlight_image(&mut mark, format!("{}{}", leftover_path, "assets/minecraft/textures/entity/sweep.png"), x, y, 32, 16);
    }
    println!("{}", out_path);
}