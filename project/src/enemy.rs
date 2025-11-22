// enemy.rs
use raylib::prelude::*;
use crate::maze::Maze;

#[derive(Clone, Copy)]
pub enum TurnPreference {
    Left,
    Right,
}

pub struct Enemy {
    pub pos: Vector2,
    pub texture_key: char,
    animation_timer: f32,
    velocity: Vector2,
    turn_preference: TurnPreference,
    speed: f32, //velocidad del enemigo
}

impl Enemy {
    pub fn new(x: f32, y: f32, turn_preference: TurnPreference, speed: f32) -> Self {
        Enemy {
            pos: Vector2::new(x, y),
            texture_key: 'e',
            animation_timer: 0.0,
            velocity: Vector2::new(1.0, 0.0),
            turn_preference,
            speed,
        }
    }

    pub fn update(&mut self, delta_time: f32, maze: &Maze, block_size: usize) {
        self.animation_timer += delta_time;
        if self.animation_timer > 0.4 {
            self.animation_timer = 0.0;
            self.texture_key = if self.texture_key == 'e' { 'f' } else { 'e' };
        }

        let check_pos = self.pos + self.velocity * (block_size as f32 / 4.0);
        let grid_x = check_pos.x as usize / block_size;
        let grid_y = check_pos.y as usize / block_size;
        
        let is_wall_ahead = grid_y >= maze.len()
            || grid_x >= maze[grid_y].len()
            || maze[grid_y][grid_x] != ' ';

        if is_wall_ahead {
            let right_dir = Vector2::new(self.velocity.y, -self.velocity.x);
            let left_dir = Vector2::new(-self.velocity.y, self.velocity.x);
            let back_dir = self.velocity * -1.0;

            let is_clear = |dir: Vector2| -> bool {
                let check = self.pos + dir * (block_size as f32 / 2.0);
                let gx = check.x as usize / block_size;
                let gy = check.y as usize / block_size;
                gy < maze.len() && gx < maze[gy].len() && maze[gy][gx] == ' '
            };

            let (preferred_dir, unpreferred_dir) = match self.turn_preference {
                TurnPreference::Right => (right_dir, left_dir),
                TurnPreference::Left => (left_dir, right_dir),
            };

            if is_clear(preferred_dir) {
                self.velocity = preferred_dir;
            } else if is_clear(unpreferred_dir) {
                self.velocity = unpreferred_dir;
            } else {
                self.velocity = back_dir;
            }
        }
        
        self.pos += self.velocity * self.speed * delta_time;
    }
}