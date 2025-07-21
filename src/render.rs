use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};


pub fn render(
    framebuffer: &mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_current_color(Color::GREEN);
    bhm_line(
        framebuffer,
        LineaBonita::new((50.0 + translate_x) as i32, (50.0 + translate_y) as i32),
        LineaBonita::new((350.0 + translate_x) as i32, (350.0 + translate_y) as i32),
        3,
    );

    framebuffer.set_current_color(Color::RED);
    bhm_line(
        framebuffer,
        LineaBonita::new((350.0 + translate_x) as i32, (50.0 + translate_y) as i32),
        LineaBonita::new((50.0 + translate_x) as i32, (350.0 + translate_y) as i32),
        3,
    );
}
