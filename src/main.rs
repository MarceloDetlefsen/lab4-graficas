mod framebuffer;
mod line;
mod obj;
mod render;

use framebuffer::Framebuffer;
use obj::load_obj;
use render::{fit_to_screen, render_obj};
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let margin = 0.1;

    let nave_path = "assets/Nave.obj";
    println!("Cargando modelo 3D: '{}'...", nave_path);

    let nave = load_obj(nave_path).expect("No se pudo leer assets/Nave.obj");
    println!("Vértices cargados:   {}", nave.vertices.len());
    println!("Triángulos cargados: {}", nave.indices.len() / 3);

    // Obtener límites mediante el método bounds de Obj
    let (min, max) = nave.bounds();
    println!(
        "Bounding box: min = ({:.2}, {:.2}, {:.2}), max = ({:.2}, {:.2}, {:.2})",
        min.x, min.y, min.z, max.x, max.y, max.z
    );

    // Calcular escala y offset automáticos
    let (scale, offset) = fit_to_screen(&nave, width, height, margin);
    println!(
        "Ajuste a pantalla (margin = {:.1}): scale = {:.3}, offset = ({:.2}, {:.2})",
        margin, scale, offset.x, offset.y
    );

    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);
    framebuffer.clear();

    println!("Renderizando modelo...");
    render_obj(&mut framebuffer, &nave, scale, offset);

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
    println!("Modelo exportado exitosamente a '{}'", output_file);
}
