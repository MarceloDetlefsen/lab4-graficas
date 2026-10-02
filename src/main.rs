mod framebuffer;
mod line;
mod obj;
mod render;

use framebuffer::Framebuffer;
use obj::load_obj;
use render::render_obj;
use raylib::prelude::*;

fn main() {
    let nave_path = "assets/Nave.obj";
    println!("Cargando modelo 3D: '{}'...", nave_path);

    let nave = load_obj(nave_path).expect("No se pudo leer assets/Nave.obj");
    println!("Vértices cargados:   {}", nave.vertices.len());
    println!("Triángulos cargados: {}", nave.indices.len() / 3);

    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);
    framebuffer.clear();

    // Escala y offset fijos provisionales ajustados a ojo para centrar la nave
    let scale = 18.0;
    let offset = Vector2::new(450.0, 330.0);

    println!("Renderizando modelo con render_obj...");
    render_obj(&mut framebuffer, &nave, scale, offset);

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("Modelo exportado exitosamente a '{}'", output_file);
}
