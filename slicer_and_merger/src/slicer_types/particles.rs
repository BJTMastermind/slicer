use crate::util::hightlight_image;
use image::{GenericImage, RgbaImage};
use std::fs;

pub fn slice_particles(from_path: &str, to_path: &str, leftover_path: Option<&str>) {
    let textures = vec![
        ("generic_0", 0, 0, 8, 8),
        ("generic_1", 8, 0, 8, 8),
        ("generic_2", 16, 0, 8, 8),
        ("generic_3", 24, 0, 8, 8),
        ("generic_4", 32, 0, 8, 8),
        ("generic_5", 40, 0, 8, 8),
        ("generic_6", 48, 0, 8, 8),
        ("generic_7", 56, 0, 8, 8),
        ("splash_0", 24, 8, 8, 8),
        ("splash_1", 32, 8, 8, 8),
        ("splash_2", 40, 8, 8, 8),
        ("splash_3", 48, 8, 8, 8),
        ("sga_a", 8, 112, 8, 8),
        ("sga_b", 16, 112, 8, 8),
        ("sga_c", 24, 112, 8, 8),
        ("sga_d", 32, 112, 8, 8),
        ("sga_e", 40, 112, 8, 8),
        ("sga_f", 48, 112, 8, 8),
        ("sga_g", 56, 112, 8, 8),
        ("sga_h", 64, 112, 8, 8),
        ("sga_i", 72, 112, 8, 8),
        ("sga_j", 80, 112, 8, 8),
        ("sga_k", 88, 112, 8, 8),
        ("sga_l", 96, 112, 8, 8),
        ("sga_m", 104, 112, 8, 8),
        ("sga_n", 112, 112, 8, 8),
        ("sga_o", 120, 112, 8, 8),
        ("sga_p", 0, 120, 8, 8),
        ("sga_q", 8, 120, 8, 8),
        ("sga_r", 16, 120, 8, 8),
        ("sga_s", 24, 120, 8, 8),
        ("sga_t", 32, 120, 8, 8),
        ("sga_u", 40, 120, 8, 8),
        ("sga_v", 48, 120, 8, 8),
        ("sga_w", 56, 120, 8, 8),
        ("sga_x", 64, 120, 8, 8),
        ("sga_y", 72, 120, 8, 8),
        ("sga_z", 80, 120, 8, 8),
        ("effect_0", 0, 64, 8, 8),
        ("effect_1", 8, 64, 8, 8),
        ("effect_2", 16, 64, 8, 8),
        ("effect_3", 24, 64, 8, 8),
        ("effect_4", 32, 64, 8, 8),
        ("effect_5", 40, 64, 8, 8),
        ("effect_6", 48, 64, 8, 8),
        ("effect_7", 56, 64, 8, 8),
        ("glitter_0", 0, 88, 8, 8),
        ("glitter_1", 8, 88, 8, 8),
        ("glitter_2", 16, 88, 8, 8),
        ("glitter_3", 24, 88, 8, 8),
        ("glitter_4", 32, 88, 8, 8),
        ("glitter_5", 40, 88, 8, 8),
        ("glitter_6", 48, 88, 8, 8),
        ("glitter_7", 56, 88, 8, 8),
        ("spark_0", 0, 80, 8, 8),
        ("spark_1", 8, 80, 8, 8),
        ("spark_2", 16, 80, 8, 8),
        ("spark_3", 24, 80, 8, 8),
        ("spark_4", 32, 80, 8, 8),
        ("spark_5", 40, 80, 8, 8),
        ("spark_6", 48, 80, 8, 8),
        ("spark_7", 56, 80, 8, 8),
        ("spell_0", 0, 72, 8, 8),
        ("spell_1", 8, 72, 8, 8),
        ("spell_2", 16, 72, 8, 8),
        ("spell_3", 24, 72, 8, 8),
        ("spell_4", 32, 72, 8, 8),
        ("spell_5", 40, 72, 8, 8),
        ("spell_6", 48, 72, 8, 8),
        ("spell_7", 56, 72, 8, 8),
        ("bubble_pop_0", 0, 131, 16, 16),
        ("bubble_pop_1", 16, 131, 16, 16),
        ("bubble_pop_2", 32, 131, 16, 16),
        ("bubble_pop_3", 48, 131, 16, 16),
        ("bubble_pop_4", 64, 131, 16, 16),
        ("flash", 32, 16, 32, 32),
        ("nautilus", 0, 104, 8, 8),
        ("note", 0, 32, 8, 8),
        ("angry", 8, 40, 8, 8),
        ("bubble", 0, 16, 8, 8),
        ("damage", 24, 32, 8, 8),
        ("flame", 0, 24, 8, 8),
        ("lava", 8, 24, 8, 8),
        ("heart", 0, 40, 8, 8),
        ("glint", 16, 40, 8, 8),
        ("enchanted_hit", 16, 32, 8, 8),
        ("critical_hit", 8, 32, 8, 8),
        ("drip_hang", 0, 56, 8, 8),
        ("drip_fall", 8, 56, 8, 8),
        ("drip_land", 16, 56, 8, 8),
        ("fishing_hook", 8, 16, 8, 8),
    ];
    if let Some(lp) = leftover_path {
        let used_path = format!(
            "{}{}",
            lp, "assets/minecraft/textures/particle/particles.png"
        );
        fs::copy(
            format!(
                "{}{}",
                from_path, "assets/minecraft/textures/particle/particles.png"
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
            textures[i].3,
            textures[i].4,
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
    w: u32,
    h: u32,
) {
    let mut base: RgbaImage = image::open(format!(
        "{}{}",
        from_path, "assets/minecraft/textures/particle/particles.png"
    ))
    .unwrap()
    .to_rgba8();
    let mut mark: RgbaImage;
    let out_path: String;
    if &name == &"fishing_hook" {
        out_path = format!(
            "{}{}{}{}",
            to_path, "assets/minecraft/textures/entity/", &name, ".png"
        );
    } else {
        out_path = format!(
            "{}{}{}{}",
            to_path, "assets/minecraft/textures/particle/", &name, ".png"
        );
    }
    let texture: RgbaImage = base.sub_image(x, y, w, h).to_image();
    texture.save(&out_path).unwrap();
    if let Some(lp) = leftover_path {
        mark = image::open(format!(
            "{}{}",
            lp, "assets/minecraft/textures/particle/particles.png"
        ))
        .unwrap()
        .to_rgba8();
        hightlight_image(
            &mut mark,
            format!(
                "{}{}",
                lp, "assets/minecraft/textures/particle/particles.png"
            ),
            x,
            y,
            w,
            h,
        );
    }
    println!("{}", out_path);
}
