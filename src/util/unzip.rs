use image::RgbaImage;
use std::{fs, io::Read};
use crate::util::*;
use zip::ZipArchive;

pub fn read_from_zip(in_path: &String, out_path: &str, leftover: Option<&str>) {
    let fname = std::path::Path::new(in_path.as_str());
    let file = fs::File::open(&fname).unwrap();

    if let Ok(mut zip) = ZipArchive::new(file) {
        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            if file.is_file() {
                let check_name = file.name().to_string().split("/").collect::<Vec<_>>().last().map(ToString::to_string).unwrap();
                match check_name.as_str() {
                    "explosion.png" => {
                        let img = get_image(file);
                        ImageData::default().slice_image(&img, check_name.as_str(), out_path, leftover, &PackImagePaths::Explosion);
                    },
                    "sweep.png" => {
                        let img = get_image(file);
                        ImageData::default().slice_image(&img, check_name.as_str(), out_path, leftover, &PackImagePaths::Sweep);
                    },
                    "inventory.png" => {
                        let img = get_image(file);
                        ImageData::default().slice_image(&img, check_name.as_str(), out_path, leftover, &PackImagePaths::Effects);
                    },
                    "paintings_kristoffer_zetterstrand.png" => {
                        let img = get_image(file);
                        ImageData::default().slice_image(&img, check_name.as_str(), out_path, leftover, &PackImagePaths::Paintings);
                    },
                    "particles.png" => {
                        let img = get_image(file);
                        ImageData::default().slice_image(&img, check_name.as_str(), out_path, leftover, &PackImagePaths::Particles);
                    },
                    _ => (),
                }
            }
        }
    }
}

fn get_image(file: zip::read::ZipFile) -> RgbaImage {
    let data = &file.bytes().filter_map(|c| c.ok()).map(|c| c.clone()).collect::<Vec<u8>>();
    let img_data = image::load_from_memory(&data).unwrap();
    let img = img_data.into_rgba8();

    img
}