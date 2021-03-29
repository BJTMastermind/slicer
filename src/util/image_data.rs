use crate::util::hightlight_image;
use image::{GenericImage, RgbaImage};
use PackImagePaths::*;
use std::fs;

pub trait Slicer {
    fn slice(
        &self,
        from_path: &str,
        to_path: &str,
        leftover_path: Option<&str>,
        pip: &PackImagePaths,
    );
    fn slice_all(&self, from_path: &str, to_path: &str, leftover_path: Option<&str>) {
        use PackImagePaths::*;
        for pip in vec![Paintings, Effects, Particles, Explosion, Sweep] {
            self.slice(from_path, to_path, leftover_path, &pip);
        }
    }
}

pub trait Merger {
    fn merge(&self);
    fn merge_all(&self) {}
}

pub enum PackImagePaths {
    Paintings,
    Effects,
    Particles,
    Explosion,
    Sweep,
}

impl PackImagePaths {
    const PATH: &'static str = "assets/minecraft/textures";

    fn atlas_path(&self) -> String {
        match self {
            Self::Effects => format!("{}/gui/container/inventory.png", Self::PATH),
            Self::Explosion => format!("{}/entity/explosion.png", Self::PATH),
            Self::Paintings => format!(
                "{}/painting/paintings_kristoffer_zetterstrand.png",
                Self::PATH
            ),
            Self::Particles => format!("{}/particle/particles.png", Self::PATH),
            Self::Sweep => format!("{}/entity/sweep.png", Self::PATH),
        }
    }

    fn resource_path(&self) -> String {
        use PackImagePaths::*;
        match self {
            Effects => format!("{}/mob_effect/", Self::PATH),
            Paintings => format!("{}/painting/", Self::PATH),
            _ => format!("{}/particle/", Self::PATH),
        }
    }
}

pub struct ImageData {
    paintings: Vec<(&'static str, u8, u8, u8, u8)>,
    effects: Vec<(&'static str, u8, u8, u8, u8)>,
    particles: Vec<(&'static str, u8, u8, u8, u8)>,
    explosions: Vec<(&'static str, u8, u8, u8, u8)>,
    sweep: Vec<(&'static str, u8, u8, u8, u8)>,
}

impl Default for ImageData {
    fn default() -> Self {       
        let paintings = vec![
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
            ("donkey_kong", 192, 112, 64, 48),
        ];
        let effects = vec![
            ("speed", 0, 198, 18, 18),
            ("slowness", 18, 198, 18, 18),
            ("haste", 36, 198, 18, 18),
            ("mining_fatigue", 54, 198, 18, 18),
            ("strength", 72, 198, 18, 18),
            ("jump_boost", 36, 216, 18, 18),
            ("nausea", 54, 216, 18, 18),
            ("regenertion", 126, 198, 18, 18),
            ("resistance", 108, 216, 18, 18),
            ("fire_resistance", 126, 216, 18, 18),
            ("water_breathing", 0, 234, 18, 18),
            ("invisibility", 0, 216, 18, 18),
            ("blindness", 90, 216, 18, 18),
            ("night_vision", 72, 216, 18, 18),
            ("hunger", 18, 216, 18, 18),
            ("weakness", 90, 198, 18, 18),
            ("poison", 108, 198, 18, 18),
            ("wither", 18, 234, 18, 18),
            ("health_boost", 126, 234, 18, 18),
            ("absorption", 36, 234, 18, 18),
            ("glowing", 72, 234, 18, 18),
            ("levitation", 54, 234, 18, 18),
            ("luck", 90, 234, 18, 18),
            ("unluck", 108, 234, 18, 18),
            ("slow_falling", 144, 198, 18, 18),
            ("conduit_power", 162, 198, 18, 18),
            ("dolphins_grace", 180, 198, 18, 18),
        ];
        let particles = vec![
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
        let explosions = vec![
            ("explosion_0", 0, 0, 32, 32),
            ("explosion_1", 32, 0, 32, 32),
            ("explosion_2", 64, 0, 32, 32),
            ("explosion_3", 96, 0, 32, 32),
            ("explosion_4", 0, 32, 32, 32),
            ("explosion_5", 32, 32, 32, 32),
            ("explosion_6", 64, 32, 32, 32),
            ("explosion_7", 96, 32, 32, 32),
            ("explosion_8", 0, 64, 32, 32),
            ("explosion_9", 32, 64, 32, 32),
            ("explosion_10", 64, 64, 32, 32),
            ("explosion_11", 96, 64, 32, 32),
            ("explosion_12", 0, 96, 32, 32),
            ("explosion_13", 32, 96, 32, 32),
            ("explosion_14", 64, 96, 32, 32),
            ("explosion_15", 96, 96, 32, 32),
        ];
        let sweep = vec![
            ("sweep_0", 0, 0, 32, 16),
            ("sweep_1", 32, 0, 32, 16),
            ("sweep_2", 64, 0, 32, 16),
            ("sweep_3", 96, 0, 32, 16),
            ("sweep_4", 0, 16, 32, 16),
            ("sweep_5", 32, 16, 32, 16),
            ("sweep_6", 64, 16, 32, 16),
            ("sweep_7", 96, 16, 32, 16),
        ];
        Self {
            paintings,
            effects,
            particles,
            explosions,
            sweep,
        }
    }
}

impl Slicer for ImageData {
    fn slice(&self, from_path: &str, to_path: &str, leftover_path: Option<&str>, pip: &PackImagePaths) {
        if let Some(lp) = leftover_path {
            let used_path = format!("{}{}", lp, pip.atlas_path());
            fs::copy(format!("{}{}", from_path, pip.atlas_path()), used_path).expect("Could not find copy image for leftover.");
        }
        let textures = match pip {
            Effects => &self.effects,
            Explosion => &self.explosions,
            Paintings => &self.paintings,
            Particles => &self.particles,
            Sweep => &self.sweep,
        };
        for name_dim in textures.iter() {
            get_image(from_path, to_path, leftover_path, name_dim, pip);
        }
    }
}

fn get_image(from_path: &str, to_path: &str, leftover_path: Option<&str>, (name, x, y, width, height): &(&str, u8, u8, u8, u8), pip: &PackImagePaths) {
    let mut base: RgbaImage = image::open(format!("{}{}", from_path, pip.atlas_path())).unwrap().to_rgba8();
    let mut mark: RgbaImage;
    let out_path: String;
    if name == &"fishing_hook" {
        out_path = format!("{}{}{}{}", to_path, "assets/minecraft/textures/entity/", &name, ".png");
    } else {
        out_path = format!("{}{}{}{}", to_path, pip.resource_path(), &name, ".png");
    }
    let texture: RgbaImage = base.sub_image(*x as u32, *y as u32, *width as u32, *height as u32).to_image();
    if let Some(lp) = leftover_path {
        mark = image::open(format!("{}{}", lp, pip.atlas_path())).unwrap().to_rgba8();
        hightlight_image(&mut mark, format!("{}{}", lp, pip.atlas_path()), *x as u32,
            *y as u32,
            *width as u32,
            *height as u32,
        );
    }
    match texture.save(&out_path) {
        Ok(_) => {}
        Err(error) => println!("Could not save texture: {}", error),
    }
    println!("{}", out_path);
}
