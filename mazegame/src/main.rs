// src/main.rs

use raylib::prelude::*;
use std::f32::consts::FRAC_PI_4;

mod maze;
mod render3d;
mod player;
mod caster;
mod controller;
mod textures;
mod framebuffer;

use maze::{load_maze, render_maze, find_first_free_cell};
use render3d::render3d;
use player::Player;
use caster::cast_ray;
use controller::process_input;
use textures::TextureManager;
use framebuffer::{Framebuffer, calc_block_size_offset};

fn main() {
    let window_width: i32 = 1000;
    let window_height: i32 = 800;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Laberinto Verde")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    // Framebuffer
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let texman = TextureManager::new(&mut window, &raylib_thread);

    // Cargar mapa desde archivo txt
    let maze = load_maze("maze.txt");

    // Tamaño de bloque y offsets del mapa en pantalla
    let (block, offset_x, offset_y) =
        calc_block_size_offset(&maze, window_width as u32, window_height as u32);

    // Elegir spawn en celda libre
    let (spawn_x, spawn_y) = find_first_free_cell(&maze).unwrap_or((0, 0));
    
    // Crear jugador
    let mut player = Player {
        pos: Vector2::new(
            spawn_x as f32 + 0.5, spawn_y as f32 + 0.5
        ),
        a: FRAC_PI_4, // angulo de vista inicial (45)
    };

    render_maze(&mut framebuffer, &maze, block, offset_x, offset_y);

    while  !window.window_should_close() {
        // inicializar ventana
        let dt = window.get_frame_time();

        // procesar teclas WASD y flechas
        process_input(&window, &mut player, &maze, dt);

        //limpiar framebuffer antes de dibujar
        framebuffer.clear();

        // Dibujar laberinto 3D
        render3d(&mut framebuffer, &maze, &player, block as usize, offset_x, offset_y, &texman);

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::BLACK);
        
        // Dibujar laberinto
        framebuffer.draw_maze(&mut d, &raylib_thread);

        // Dibujar jugador
        let px = (offset_x as f32 + player.pos.x * block as f32) as i32;
        let py = (offset_y as f32 + player.pos.y * block as f32) as i32;
        
        framebuffer.draw_player(px, py);
    }
    
}
