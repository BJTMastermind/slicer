use image::{Rgba, RgbaImage};

pub fn hightlight_image(image: &mut RgbaImage, name: String, x: u32, y: u32, w: u32, h: u32) {
    for img_y in y..(y + h) {
        for img_x in x..(x + w) {
            let pixel = image.get_pixel(img_x, img_y);
            let color: Rgba<u8>;
            if pixel[1] == 255 || pixel[2] == 255 {
                color = Rgba([
                    pixel[0].saturating_add(128),
                    128,
                    128,
                    pixel[3].saturating_add(128),
                ]);
            } else {
                color = Rgba([
                    pixel[0].saturating_add(128),
                    pixel[1],
                    pixel[2],
                    pixel[3].saturating_add(128),
                ]);
            }
            image.put_pixel(img_x, img_y, color);
        }
    }
    image.save(name).unwrap();
}
