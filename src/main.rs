mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::{point, triangle};
use raylib::prelude::*;

/// Función que recibe un Vector3 e imprime sus tres componentes.
fn imprimir_vertice(v: Vector3) {
    println!("Vector3 -> x: {}, y: {}, z: {}", v.x, v.y, v.z);
}

fn main() {
    // --- Ejercicio con vectores ---
    let v1 = Vector2::new(100.0, 200.0);
    println!("Primer vértice (Vector2): ({}, {})", v1.x, v1.y);

    let v2 = Vector3::new(3.0, 5.0, -2.0);
    println!("Componentes individuales del segundo vértice (Vector3):");
    println!("  x = {}", v2.x);
    println!("  y = {}", v2.y);
    println!("  z = {}", v2.z);

    println!("\nLlamando a imprimir_vertice:");
    imprimir_vertice(v2);

    // --- Framebuffer ---
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.clear();

    // Dibujar un punto usando Vector2
    framebuffer.set_current_color(Color::RED);
    point(&mut framebuffer, v1);

    // --- Dibujar triángulo ---
    let a = Vector2::new(320.0, 100.0);
    let b = Vector2::new(150.0, 350.0);
    let c = Vector2::new(500.0, 350.0);

    framebuffer.set_current_color(Color::GREEN);
    triangle(&mut framebuffer, a, b, c);

    // Triángulo adicional para probar distinta escala y orientación
    let d = Vector2::new(600.0, 120.0);
    let e = Vector2::new(720.0, 260.0);
    let f = Vector2::new(550.0, 300.0);

    framebuffer.set_current_color(Color::SKYBLUE);
    triangle(&mut framebuffer, d, e, f);

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("\nTriángulo(s) dibujados exitosamente y exportados a '{}'", output_file);
}
