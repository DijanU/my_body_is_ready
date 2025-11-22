//player.rs
use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events( //Comprobar si el jugador ha llegado a la meta
    window: &mut RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
    goal_unlocked: bool,
) -> bool {
    const MOVE_SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = PI / 40.0;
    const MOUSE_SENSITIVITY: f32 = 0.002; // Reducido para mejor control

    // Deshabilitar cursor para capturar el mouse
    if !window.is_cursor_hidden() {
        window.disable_cursor();
    }

    // Obtener el delta del mouse directamente aquí
    let mouse_delta = window.get_mouse_delta();
    
    //Rotación con teclado
    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    // Rotación con mouse
    player.a += mouse_delta.x * MOUSE_SENSITIVITY;

    // Normalizar el ángulo para evitar overflow
    if player.a > PI {
        player.a -= 2.0 * PI;
    } else if player.a < -PI {
        player.a += 2.0 * PI;
    }

    let mut next_pos = player.pos;
    let mut moved = false;

    //Movimiento con teclado
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    // Movimiento con mouse (botones)
    if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    // Verificar colisiones y movimiento
    if moved {
        let grid_x = next_pos.x as usize / block_size;
        let grid_y = next_pos.y as usize / block_size;

        if grid_y < maze.len() && grid_x < maze[grid_y].len() {
            if goal_unlocked && maze[grid_y][grid_x] == 'g' { //Comprobar si el jugador ha llegado a la meta
                return true;
            }

            if maze[grid_y][grid_x] == ' ' {
                player.pos = next_pos;
            }
        }
    }
    
    false // No se ha ganado
}