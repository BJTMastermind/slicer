use image::{RgbaImage, GenericImage};
use std::fs;
use crate::util::hightlight_image;

pub fn slice_effects(from_path: &String, to_path: &String, leftover_path: &String) {
    let textures = vec![
        ("speed", 0, 198),
        ("slowness", 18, 198),
        ("haste", 36, 198),
        ("mining_fatigue", 54, 198),
        ("strength", 72, 198),
        ("jump_boost", 36, 216),
        ("nausea", 54, 216),
        ("regenertion", 126, 198),
        ("resistance", 108, 216),
        ("fire_resistance", 126, 216),
        ("water_breathing", 0, 234),
        ("invisibility", 0, 216),
        ("blindness", 90, 216),
        ("night_vision", 72, 216),
        ("hunger", 18, 216),
        ("weakness", 90, 198),
        ("poison", 108, 198),
        ("wither", 18, 234),
        ("health_boost", 126, 234),
        ("absorption", 36, 234),
        ("glowing", 72, 234),
        ("levitation", 54, 234),
        ("luck", 90, 234),
        ("unluck", 108, 234),
        ("slow_falling", 144, 198),
        ("conduit_power", 162, 198),
        ("dolphins_grace", 180, 198)];
    if leftover_path != &"".to_string() {
        let used_path = format!("{}{}", leftover_path, "assets/minecraft/textures/gui/container/inventory.png");
        fs::copy(format!("{}{}", from_path, "assets/minecraft/textures/gui/container/inventory.png"), used_path).unwrap();
    }
    for i in 0..textures.len() {
        get_image(&from_path, &to_path, &leftover_path, textures[i].0, textures[i].1, textures[i].2);
    }
}

pub fn get_image(from_path: &String, to_path: &String, leftover_path: &String, name: &str, x: u32, y: u32) {
    let mut base: RgbaImage = image::open(format!("{}{}", from_path, "assets/minecraft/textures/gui/container/inventory.png")).unwrap().to_rgba8();
    let mut mark: RgbaImage;
    let out_path = format!("{}{}{}{}", to_path, "assets/minecraft/textures/mob_effect/", &name, ".png");
    let texture: RgbaImage = base.sub_image(x, y, 18, 18).to_image();
    texture.save(&out_path).unwrap();
    if leftover_path != &"".to_string() {
        mark = image::open(format!("{}{}", leftover_path, "assets/minecraft/textures/gui/container/inventory.png")).unwrap().to_rgba8();
        hightlight_image(&mut mark, format!("{}{}", leftover_path, "assets/minecraft/textures/gui/container/inventory.png"), x, y, 18, 18);
    }
    println!("{}", out_path);
}