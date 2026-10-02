#![allow(dead_code)]

use raylib::prelude::{Vector2, Vector3};

/// Proyecta un vértice 3D a coordenadas 2D de pantalla usando únicamente sus componentes X e Y.
/// En el espacio de modelado 3D el eje Y positivo apunta hacia arriba, mientras que en coordenadas
/// de pantalla el eje Y crece hacia abajo; por esta razón se invierte el signo de Y (-v.y).
pub fn to_screen(v: Vector3, scale: f32, offset: Vector2) -> Vector2 {
    Vector2::new(
        v.x * scale + offset.x,
        -v.y * scale + offset.y,
    )
}
