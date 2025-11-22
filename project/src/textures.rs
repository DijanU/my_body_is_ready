// textures.rs
use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    image_data: HashMap<char, (Image, u32, u32)>, // Store images, width, and height for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
}

unsafe impl Sync for TextureManager {}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut image_data = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('+', "assets/wall.png"),
            ('-', "assets/wall.png"),
            ('|', "assets/wall.png"),
            ('g', "assets/reggie.png"),
            ('e', "assets/reggie.png"),
            ('f', "assets/wii.png"),
            ('c', "assets/wii.png"),
            ('h', "assets/wii.png"),
            ('n', "assets/nintendo.png"), // Added nintendo.png
            ('d', "assets/wite_nintendo_direct.png"), // Added wite_nintendo_direct.png
            ('#', "assets/wall.png"), // default/fallback
        ];

        for (ch, path) in texture_files {
            let image = Image::load_image(path).expect(&format!("Failed to load image {}", path));
            let width = image.width as u32;
            let height = image.height as u32;
            let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
            texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
            image_data.insert(ch, (image, width, height));
            textures.insert(ch, texture);
        }

        TextureManager { image_data, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some((image, width, height)) = self.image_data.get(&ch) {
            let x = tx.min(width - 1) as i32;
            let y = ty.min(height - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }

    pub fn get_image_dimensions(&self, ch: char) -> Option<(u32, u32)> {
        self.image_data.get(&ch).map(|(_, w, h)| (*w, *h))
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);

        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}