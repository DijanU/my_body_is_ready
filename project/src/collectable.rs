// collectable.rs
use raylib::prelude::*;
use crate::textures::TextureManager;

pub struct Collectable {
    pub pos: Vector2,
    pub texture_key: char,
}

impl Collectable {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Collectable {
            pos: Vector2::new(x, y), 
            texture_key,
        }
    }
}