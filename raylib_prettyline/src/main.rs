mod framebuffer;
mod bhm_line;
mod figures;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use bhm_line::{bhm_line,LineaBonita};
use figures::dibujar_triangulo_regular;

fn main() {
    let image_width = 800;
    let image_height = 800;
    let mut framebuffer = Framebuffer::new(image_width, image_height);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    framebuffer.set_current_color(Color::CYAN);
    bhm_line(
        &mut framebuffer,
        LineaBonita::new(60, 60),
        LineaBonita::new(600, 100),
        3, //Grosor de la línea
    );

    framebuffer.set_current_color(Color::MAGENTA);
    dibujar_triangulo_regular(
        &mut framebuffer,
        LineaBonita::new(300, 100),
        100, //Radio del triángulo
        3, //Grosor de la línea
    );

    let output_file_name ="mkline_image.png";

    framebuffer.render_to_file(output_file_name);
}
