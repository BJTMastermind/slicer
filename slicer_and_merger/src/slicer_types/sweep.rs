use crate::util::hightlight_image;
use image::{GenericImage, RgbaImage};
use std::fs;

pub fn slice_sweep(from_path: &str, to_path: &str, leftover_path: Option<&str>) {
    let textures = vec![
        ("sweep_0", 0, 0),
        ("sweep_1", 32, 0),
        ("sweep_2", 64, 0),
        ("sweep_3", 96, 0),
        ("sweep_4", 0, 16),
        ("sweep_5", 32, 16),
        ("sweep_6", 64, 16),
        ("sweep_7", 96, 16),
    ];
    if let Some(lp) = leftover_path {
        let used_path = format!("{}{}", lp, "assets/minecraft/textures/entity/sweep.png");
        fs::copy(
            format!(
                "{}{}",
                from_path, "assets/minecraft/textures/entity/sweep.png"
            ),
            used_path,
        )
        .unwrap();
    }
    for i in 0..textures.len() {
        get_image(
            from_path,
            to_path,
            leftover_path,
            textures[i].0,
            textures[i].1,
            textures[i].2,
        )
    }
}

fn get_image(
    from_path: &str,
    to_path: &str,
    leftover_path: Option<&str>,
    name: &str,
    x: u32,
    y: u32,
) {
    let mut base: RgbaImage = image::open(format!(
        "{}{}",
        from_path, "assets/minecraft/textures/entity/sweep.png"
    ))
    .unwrap()
    .to_rgba8();
    let mut mark: RgbaImage;
    let out_path = format!(
        "{}{}{}{}",
        to_path, "assets/minecraft/textures/particle/", &name, ".png"
    );
    let texture: RgbaImage = base.sub_image(x, y, 32, 16).to_image();
    texture.save(&out_path).unwrap();
    if let Some(lp) = leftover_path {
        mark = image::open(format!(
            "{}{}",
            lp, "assets/minecraft/textures/entity/sweep.png"
        ))
        .unwrap()
        .to_rgba8();
        hightlight_image(
            &mut mark,
            format!("{}{}", lp, "assets/minecraft/textures/entity/sweep.png"),
            x,
            y,
            32,
            16,
        );
    }
    println!("{}", out_path);
}
