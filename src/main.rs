mod framebuffer;
mod line;

use framebuffer::{Framebuffer, point};
use raylib::prelude::*;

/// Función reto: Recibe un Vector3 e imprime sus tres componentes.
fn imprimir_vertice(v: Vector3) {
    println!("Vector3 -> x: {}, y: {}, z: {}", v.x, v.y, v.z);
}

fn main() {
    // --- Ejercicio ---
    // 1. Crea un Vector2 que represente la posición (100, 200).
    let v1 = Vector2::new(100.0, 200.0);
    println!("Primer vértice (Vector2): ({}, {})", v1.x, v1.y);

    // 2. Crea un Vector3 que represente la posición (3, 5, -2).
    let v2 = Vector3::new(3.0, 5.0, -2.0);

    // 3. Imprime individualmente las componentes x, y y z del segundo vértice.
    println!("Componentes individuales del segundo vértice (Vector3):");
    println!("  x = {}", v2.x);
    println!("  y = {}", v2.y);
    println!("  z = {}", v2.z);

    // --- Reto ---
    // Llamada a la función imprimir_vertice
    println!("\nLlamando a imprimir_vertice:");
    imprimir_vertice(v2);

    // Modificación de la función para dibujar un punto:
    // Ahora recibe un `Vector2` en lugar de `x` y `y` por separado.
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.clear();
    framebuffer.set_current_color(Color::RED);

    // Dibujar el punto usando la función que recibe Vector2
    point(&mut framebuffer, v1);
    // (o también: framebuffer.point(v1);)

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("\nPunto dibujado en la posición ({}, {}) y exportado a '{}'", v1.x, v1.y, output_file);
}
