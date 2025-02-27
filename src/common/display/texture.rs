use std::fs::File;
use std::io::Read;
use macroquad::prelude::{Texture2D, Image};

pub fn load_texture_sync(path: &str) -> Texture2D {
    let mut file = File::open(path).expect("Can't open file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Read error.");

    let image = Image::from_file_with_format(&buffer, None).unwrap();
    Texture2D::from_image(&image)
}
