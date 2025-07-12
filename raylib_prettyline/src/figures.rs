use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};

pub fn dibujar_triangulo_regular(
    framebuffer: &mut Framebuffer,
    centro: LineaBonita,
    radio: i32,
    grosor: i32,
){
    let lados = 3;
    let mut verices: Vec<LineaBonita> = Vec::new();

    for i in 0..lados {
        let angulo = 2.0 * PI * (i as f32 / lados as f32);
        let x = centro.x + (radio as f32 * angulo.cos()) as i32;
        let y = centro.y + (radio as f32 * angulo.sin()) as i32;
        verices.push(LineaBonita::new(x, y));
    }

    for i in 0..lados {
        let start = verices[i];
        let end = verices[(i + 1) % lados];
        bhm_line(framebuffer, start, end, grosor);
    }
}