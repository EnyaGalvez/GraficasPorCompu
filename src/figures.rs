// src/figures.rs

use std::f32::consts::PI;
use crate::framebuffer::Framebuffer;
use crate::bhm_line::{bhm_line, LineaBonita};

pub fn dibujar_triangulo_regular(
    framebuffer: &mut Framebuffer,
    centro: LineaBonita,
    radio: i32,
    grosor: i32,
) -> Vec<LineaBonita> {
    let lados = 3;
    let mut vertices: Vec<LineaBonita> = Vec::new();

    for i in 0..lados {
        let angulo = 2.0 * PI * (i as f32 / lados as f32);
        let x = centro.x + (radio as f32 * angulo.cos()) as i32;
        let y = centro.y + (radio as f32 * angulo.sin()) as i32;
        vertices.push(LineaBonita::new(x, y));
    }

    for i in 0..lados {
        let start = vertices[i];
        let end = vertices[(i + 1) % lados];
        bhm_line(framebuffer, start, end, grosor);
    }

    vertices
}

pub fn dibujar_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
    grosor: i32,
) {
    let a = LineaBonita::new(x, y);
    let b = LineaBonita::new(x + ancho, y);
    let c = LineaBonita::new(x + ancho, y + alto);
    let d = LineaBonita::new(x, y + alto);

    bhm_line(framebuffer, a, b, grosor); // Lado superior
    bhm_line(framebuffer, b, c, grosor); // Lado derecho
    bhm_line(framebuffer, c, d, grosor); // Lado inferior
    bhm_line(framebuffer, d, a, grosor); // Lado izquierdo
}

pub fn rellenar_poligono(
    framebuffer: &mut Framebuffer,
    vertices: &[LineaBonita],
) {
    let height = framebuffer.height as i32;

    for y in 0..height {
        let mut intersecciones = Vec::new();

        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            // Detectar intersección entre la línea y la fila actual (scanline)
            if (v1.y <= y && v2.y > y) || (v2.y <= y && v1.y > y) {
                let x = v1.x + (y - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                intersecciones.push(x);
            }
        }

        // Ordenar las intersecciones y dibujar líneas horizontales entre pares
        intersecciones.sort();
        for i in (0..intersecciones.len()).step_by(2) {
            if i + 1 < intersecciones.len() {
                let x_start = intersecciones[i];
                let x_end = intersecciones[i + 1];
                for x in x_start..=x_end {
                    framebuffer.set_pixel(x, y, framebuffer.get_current_color());
                }
            }
        }
    }
}
