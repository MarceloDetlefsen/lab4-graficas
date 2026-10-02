mod framebuffer;
mod line;
mod obj;

use framebuffer::Framebuffer;
use line::triangle;
use obj::load_obj;
use raylib::prelude::*;

/// Función que recibe un Vector3 e imprime sus tres componentes.
fn imprimir_vertice(v: Vector3) {
    println!("Vector3 -> x: {}, y: {}, z: {}", v.x, v.y, v.z);
}

fn main() {
    // ----------------------------------------------------------------
    // 1. Probar el cargador OBJ con un modelo pequeño (assets/cube.obj)
    // ----------------------------------------------------------------
    let cube_path = "assets/cube.obj";
    println!("=== Cargando '{}' ===", cube_path);
    let cube = load_obj(cube_path).expect("No se pudo leer assets/cube.obj");

    println!("¿Cuántos vértices cargaron?: {}", cube.vertices.len());
    println!("¿Cuántos índices cargaron?: {}", cube.indices.len());
    println!("Total de triángulos: {}", cube.indices.len() / 3);

    // Recuperar los tres vértices del primer triángulo usando sus índices
    if cube.indices.len() >= 3 {
        let i0 = cube.indices[0];
        let i1 = cube.indices[1];
        let i2 = cube.indices[2];

        println!("\nPrimer triángulo (índices en Vec: [{}, {}, {}]):", i0, i1, i2);
        print!("  Vértice A: ");
        imprimir_vertice(cube.vertices[i0]);
        print!("  Vértice B: ");
        imprimir_vertice(cube.vertices[i1]);
        print!("  Vértice C: ");
        imprimir_vertice(cube.vertices[i2]);
    }

    // ----------------------------------------------------------------
    // 2. Renderizado en Framebuffer (ejercicios anteriores)
    // ----------------------------------------------------------------
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.clear();

    let a = Vector2::new(320.0, 100.0);
    let b = Vector2::new(150.0, 350.0);
    let c = Vector2::new(500.0, 350.0);

    framebuffer.set_current_color(Color::GREEN);
    triangle(&mut framebuffer, a, b, c);

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("\nTriángulo de prueba exportado a '{}'", output_file);
}
