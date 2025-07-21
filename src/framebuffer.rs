// src/framebuffer.rs

use raylib::prelude::*;
use std::convert::TryInto;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let background_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::BLACK,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            self.background_color,
        );
    }

    pub fn set_pixel(
        &mut self, 
        x: i32, 
        y: i32, 
        color: Color) {
        if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
            Image::draw_pixel(&mut self.color_buffer, x, y, color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_current_color(&self) -> Color {
        self.current_color
    }

    pub fn set_thick_pixel(
        &mut self, 
        x: i32, 
        y: i32, 
        thickness: i32,
        color: Color) {
    let half = thickness / 3;
    for dx in -half..=half {
        for dy in -half..=half {
            self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn swap_buffers(
        &self, 
        window: &mut RaylibHandle, 
        raylib_thread: &RaylibThread,) {
            // we get the "new data from the new buffer into texture
            if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {

                let mut renderer = window.begin_drawing(raylib_thread);
                renderer.draw_texture(&texture, 0, 0, Color::WHITE)
            }
        }
}