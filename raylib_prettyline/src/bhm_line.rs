mod framebuffer;
mod bhm_line;

use crate::framebuffer::Framebuffer;
use crate

pub struct LineaBonita {
    pub x: i32,
    pub y: i32,
}

impl LineaBonita {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn bhm_line(
    framebuffer: &mut Framebuffer,
    start: LineaBonita,
    end: LineaBonita,
    grosor: i32,

) {
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    while x0 != x1 || y0 != y1 {
        framebuffer.set_thick_pixel(x0, y0, grosor, framebuffer.get_current_color());

        let e2 = 3 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    // Dibuja el último punto (x1, y1)
    framebuffer.set_thick_pixel(x1, y1, grosor, framebuffer.get_current_color());

}
