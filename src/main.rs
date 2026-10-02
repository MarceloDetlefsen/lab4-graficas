mod framebuffer;
mod line;
mod obj;

use std::time::Instant;
use framebuffer::Framebuffer;
use line::triangle;
use obj::load_obj;
use raylib::prelude::*;

fn main() {
    let nave_path = "assets/Nave.obj";
    println!("========================================");
    println!("Cargando modelo 3D: '{}'", nave_path);
    println!("========================================");

    let start_load = Instant::now();
    let nave = load_obj(nave_path).expect("No se pudo leer assets/Nave.obj");
    println!("Tiempo de carga: {:?}", start_load.elapsed());
    println!("Vértices: {}", nave.vertices.len());
    println!("Triángulos: {}", nave.indices.len() / 3);

    // Calcular el bounding box para centrar y escalar el modelo adecuadamente
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for v in &nave.vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
    }

    let model_width = max_x - min_x;
    let model_height = max_y - min_y;
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let width = 800;
    let height = 600;
    let padding = 50.0;

    // Escala uniforme manteniendo la relación de aspecto
    let scale_x = (width as f32 - 2.0 * padding) / model_width;
    let scale_y = (height as f32 - 2.0 * padding) / model_height;
    let scale = scale_x.min(scale_y);

    let offset_x = width as f32 / 2.0;
    let offset_y = height as f32 / 2.0;

    println!("\nParámetros de proyección 2D:");
    println!("  Centro modelo: ({:.3}, {:.3})", center_x, center_y);
    println!("  Dimensiones:   {:.3} x {:.3}", model_width, model_height);
    println!("  Factor escala: {:.3}", scale);

    // Transformar los vértices 3D a coordenadas 2D de pantalla (escalados y trasladados)
    let vertices_2d: Vec<Vector2> = nave
        .vertices
        .iter()
        .map(|v| {
            Vector2::new(
                (v.x - center_x) * scale + offset_x,
                offset_y - (v.y - center_y) * scale, // Invertir Y para que coincida con coordenadas cartesianas
            )
        })
        .collect();

    // Preparar el framebuffer
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);
    framebuffer.clear();
    framebuffer.set_current_color(Color::WHITE);

    println!("\nDibujando triángulos en wireframe...");
    let start_render = Instant::now();

    // Dibujar cada triángulo usando directamente nuestra función `triangle`
    for chunk in nave.indices.chunks_exact(3) {
        let a = vertices_2d[chunk[0]];
        let b = vertices_2d[chunk[1]];
        let c = vertices_2d[chunk[2]];
        triangle(&mut framebuffer, a, b, c);
    }

    println!("Tiempo de renderizado: {:?}", start_render.elapsed());

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("Modelo renderizado exitosamente y exportado a '{}'", output_file);
}
