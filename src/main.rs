// src/main.rs

mod framebuffer;
mod bhm_line;
mod figures;
mod render;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};
use crate::figures::{dibujar_triangulo_regular, dibujar_rectangulo, rellenar_poligono};
use crate::render::render;

fn main() {
    let window_width = 1000;
    let window_height = 800;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Figuritas de Colores")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let mut translate_x: f32 = 0.0;
    let mut translate_y: f32 = 0.0;

    while !window.window_should_close() {
        framebuffer.clear();

        translate_x += 1.0;
        translate_y += 1.0;

        framebuffer.set_current_color(Color::CYAN);
        bhm_line(
            &mut framebuffer,
            LineaBonita::new(60, 60),
            LineaBonita::new(600, 600),
            3, //Grosor de la línea
        );

        framebuffer.set_current_color(Color::MAGENTA);
        let vertices_triangulo = dibujar_triangulo_regular(
            &mut framebuffer,
            LineaBonita::new(300, 100),
            100, //Radio del triángulo
            3, //Grosor de la línea
        );

        framebuffer.set_current_color(Color::MAGENTA);
        rellenar_poligono(&mut framebuffer, &vertices_triangulo);

        framebuffer.set_current_color(Color::GREEN);
        dibujar_rectangulo(
            &mut framebuffer,
            200,
            200,
            400,
            300,
            3,
        );

        render(&mut framebuffer, translate_x, translate_y);

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::WHITE);
        framebuffer.swap_buffers(&mut d, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}


