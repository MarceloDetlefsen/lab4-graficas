#![allow(dead_code)]

use crate::framebuffer::Framebuffer;
use crate::line::triangle;
use crate::obj::Obj;
use raylib::prelude::{Color, Vector2, Vector3};

/// Proyecta un vértice 3D a coordenadas 2D de pantalla usando únicamente sus componentes X e Y.
/// En el espacio de modelado 3D el eje Y positivo apunta hacia arriba, mientras que en coordenadas
/// de pantalla el eje Y crece hacia abajo; por esta razón se invierte el signo de Y (-v.y).
pub fn to_screen(v: Vector3, scale: f32, offset: Vector2) -> Vector2 {
    Vector2::new(
        v.x * scale + offset.x,
        -v.y * scale + offset.y,
    )
}

/// Renderiza en wireframe todos los triángulos de un objeto OBJ proyectados a 2D.
pub fn render_obj(framebuffer: &mut Framebuffer, obj: &Obj, scale: f32, offset: Vector2) {
    framebuffer.set_current_color(Color::WHITE);

    for chunk in obj.indices.chunks_exact(3) {
        let v0 = obj.vertices[chunk[0]];
        let v1 = obj.vertices[chunk[1]];
        let v2 = obj.vertices[chunk[2]];

        let p0 = to_screen(v0, scale, offset);
        let p1 = to_screen(v1, scale, offset);
        let p2 = to_screen(v2, scale, offset);

        triangle(framebuffer, p0, p1, p2);
    }
}
