use crate::ColoredChar;
use image::{ImageBuffer, Rgba};
use rusttype::{Font, Scale, point};

pub fn create_image_from_ascii(ascii: &Vec<Vec<ColoredChar>>, output_path: &str) {
   let font_data = include_bytes!("../fonts/JetBrainsMono-VariableFont_wght.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).expect("Couldn't parse the font");
    let scale = Scale::uniform(12.0); 

    let char_width = 8;
    let char_height = 12;

    let img_width = ascii[0].len() as u32 * char_width;
    let img_height = ascii.len() as u32 * char_height;

    let mut image = ImageBuffer::from_pixel(img_width, img_height, Rgba([255, 255, 255, 255]));

    for (j, row) in ascii.iter().enumerate() {
        for (i, colored_char) in row.iter().enumerate() {
            let x = (i as u32 * char_width) as i32;
            let y = ((j + 1) as u32 * char_height - 3) as i32;

            let glyph = font
                .glyph(colored_char.character)
                .scaled(scale)
                .positioned(point(x as f32, y as f32));
            
            glyph.draw(|gx, gy, v| {
                let px = x + gx as i32;
                let py = y + gy as i32;
                if px >= 0 && py >= 0 && px < img_width as i32 && py < img_height as i32 {
                    let pixel = image.get_pixel_mut(px as u32, py as u32);
                    let alpha = v;
                    *pixel = Rgba([
                        ((1.0 - alpha) * 255.0 + alpha * colored_char.r as f32) as u8,
                        ((1.0 - alpha) * 255.0 + alpha * colored_char.g as f32) as u8,
                        ((1.0 - alpha) * 255.0 + alpha * colored_char.b as f32) as u8,
                        255,
                    ]);
                }
            });
        }
    }

    image.save(output_path).expect("Couldn't save image");
}