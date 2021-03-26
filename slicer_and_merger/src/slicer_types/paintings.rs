use image::{RgbaImage, GenericImage};
use std::fs;
use crate::util::hightlight_image;

pub fn slice_paintings(from_path: &String, to_path: &String, leftover_path: &String) {
    let textures = vec![
        ("back", 240, 0, 16, 16),
        
        ("kebab", 0, 0, 16, 16),
        ("aztec", 16, 0, 16, 16),
        ("alban", 32, 0, 16, 16),
        ("aztec2", 48, 0, 16, 16),
        ("bomb", 64, 0, 16, 16),
        ("plant", 80, 0, 16, 16),
        ("wasteland", 96, 0, 16, 16),
        ("pool", 0, 32, 32, 16),
        ("courbet", 32, 32, 32, 16),
        ("sea", 64, 32, 32, 16),
        ("sunset", 96, 32, 32, 16),
        ("creebet", 128, 32, 32, 16),
        ("wanderer", 0, 64, 16, 32),
        ("graham", 16, 64, 16, 32),
        ("match", 0, 128, 32, 32),
        ("bust", 32, 128, 32, 32),
        ("stage", 64, 128, 32, 32),
        ("void", 96, 128, 32, 32),
        ("skull_and_roses", 128, 128, 32, 32),
        ("wither", 160, 128, 32, 32),
        ("fighters", 0, 96, 64, 32),
        ("pointer", 0, 192, 64, 64),
        ("pigscene", 64, 192, 64, 64),
        ("burning_skull", 128, 192, 64, 64),
        ("skeleton", 192, 64, 64, 48),
        ("donkey_kong", 192, 112, 64, 48)];
    if leftover_path != &"".to_string() {
        let used_path = format!("{}{}", leftover_path, "assets/minecraft/textures/painting/paintings_kristoffer_zetterstrand.png");
        fs::copy(format!("{}{}", from_path, "assets/minecraft/textures/painting/paintings_kristoffer_zetterstrand.png"), used_path).unwrap();
    }
    for i in 0..textures.len() {
        get_image(&from_path, &to_path, &leftover_path, textures[i].0, textures[i].1, textures[i].2, textures[i].3, textures[i].4);
    }
}

fn get_image(from_path: &String, to_path: &String, leftover_path: &String, name: &str, x: u32, y: u32, w: u32, h: u32) {
    let mut base: RgbaImage = image::open(format!("{}{}", from_path, "assets/minecraft/textures/painting/paintings_kristoffer_zetterstrand.png")).unwrap().to_rgba8();
    let mut mark: RgbaImage;
    let out_path = format!("{}{}{}{}", to_path, "assets/minecraft/textures/painting/", &name, ".png");
    let texture: RgbaImage = base.sub_image(x, y, w, h).to_image();
    if leftover_path != &"".to_string() {
        mark = image::open(format!("{}{}", leftover_path, "assets/minecraft/textures/painting/paintings_kristoffer_zetterstrand.png")).unwrap().to_rgba8();
        hightlight_image(&mut mark, format!("{}{}", leftover_path, "assets/minecraft/textures/painting/paintings_kristoffer_zetterstrand.png"), x, y, w, h);
    }
    texture.save(&out_path).unwrap();
    println!("{}", out_path);
}