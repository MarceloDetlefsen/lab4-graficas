mod framebuffer;
mod line;
mod obj;

use std::time::Instant;
use framebuffer::Framebuffer;
use line::triangle;
use obj::load_obj;
use raylib::prelude::*;

/// Función que recibe un Vector3 e imprime sus tres componentes.
fn imprimir_vertice(v: Vector3) {
    println!("Vector3 -> x: {:.4}, y: {:.4}, z: {:.4}", v.x, v.y, v.z);
}

fn main() {
    // ----------------------------------------------------------------
    // 1. Cargar y verificar el modelo Nave.obj (assets/Nave.obj)
    // ----------------------------------------------------------------
    let nave_path = "assets/Nave.obj";
    println!("========================================");
    println!("Cargando modelo 3D: '{}'", nave_path);
    println!("========================================");

    let start_time = Instant::now();
    let nave = load_obj(nave_path).expect("No se pudo leer assets/Nave.obj");
    let elapsed = start_time.elapsed();

    println!("Tiempo de carga: {:?}", elapsed);
    println!("Total de vértices cargados: {}", nave.vertices.len());
    println!("Total de índices cargados:  {}", nave.indices.len());
    let total_triangulos = nave.indices.len() / 3;
    println!("Total de triángulos:        {}", total_triangulos);

    // Verificación de límites (asegurar que todos los índices sean válidos)
    let max_indice = nave.indices.iter().copied().max().unwrap_or(0);
    println!("Índice máximo referenciado: {} (de {} disponibles)", max_indice, nave.vertices.len());
    assert!(max_indice < nave.vertices.len(), "¡Error: Hay índices fuera de rango!");

    // Bounding box (caja envolvente del modelo)
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;

    for v in &nave.vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
        min_z = min_z.min(v.z);
        max_z = max_z.max(v.z);
    }

    println!("\nDimensiones / Bounding Box de la Nave:");
    println!("  X: [{:.3}, {:.3}] (Ancho:  {:.3})", min_x, max_x, max_x - min_x);
    println!("  Y: [{:.3}, {:.3}] (Alto:   {:.3})", min_y, max_y, max_y - min_y);
    println!("  Z: [{:.3}, {:.3}] (Prof.:  {:.3})", min_z, max_z, max_z - min_z);

    // Muestra de los primeros 3 triángulos con sus vértices recuperados
    println!("\nPrimeros 3 triángulos cargados:");
    for tri_idx in 0..3.min(total_triangulos) {
        let i0 = nave.indices[tri_idx * 3];
        let i1 = nave.indices[tri_idx * 3 + 1];
        let i2 = nave.indices[tri_idx * 3 + 2];

        println!("  Triángulo #{}: índices [{}, {}, {}]", tri_idx + 1, i0, i1, i2);
        print!("    A: ");
        imprimir_vertice(nave.vertices[i0]);
        print!("    B: ");
        imprimir_vertice(nave.vertices[i1]);
        print!("    C: ");
        imprimir_vertice(nave.vertices[i2]);
    }

    // ----------------------------------------------------------------
    // 2. Renderizado en Framebuffer de prueba
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
    println!("\nFramebuffer de prueba exportado a '{}'", output_file);
}
