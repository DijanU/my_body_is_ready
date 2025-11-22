//caster.rs
use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;
use crate::framebuffer::Framebuffer;
use crate::textures::TextureManager;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
}

pub fn cast_ray_intersect(
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    texture_manager: &TextureManager, // Added TextureManager
) -> Intersect {
    let mut d = 0.0;
    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;
        
        if maze[j][i] != ' ' {
            let hitx = x - i * block_size;
            let hity = y - j * block_size;
            let mut maxhit = hity;

            if 1 < hitx && hitx < block_size - 1 {
                maxhit = hitx;
            }
            
            let (tex_width, _) = texture_manager.get_image_dimensions(maze[j][i]).unwrap_or((128, 128)); // Get actual width
            let x = ((maxhit as f32 * tex_width as f32) / block_size as f32) as usize; // Use actual width
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: x,
            }; 
        }
        
        d += 1.0;
    }
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw: bool,
    texture_manager: &TextureManager, // Added TextureManager
) -> Intersect {
    let intersect = cast_ray_intersect(maze, player, a, block_size, texture_manager);

    if draw {
        let mut d = 0.0;
        while d < intersect.distance {
            let cos = d * a.cos();
            let sin = d * a.sin();
            let x = (player.pos.x + cos) as i32;
            let y = (player.pos.y + sin) as i32;
            framebuffer.set_pixel(x, y, Color::WHITE);
            d += 1.0;
        }
    }
    
    intersect
}
