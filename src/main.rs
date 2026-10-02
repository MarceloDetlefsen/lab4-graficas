mod framebuffer;
mod line;
mod obj;
mod render;

use std::env;
use std::process;
use std::time::Instant;

use framebuffer::Framebuffer;
use obj::load_obj;
use render::{fit_to_screen, render_obj};
use raylib::prelude::*;

fn main() {
    // 1. Argumento opcional de CLI (por defecto assets/Nave.obj)
    let obj_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/Nave.obj".to_string());

    // 2. Cargar modelo midiendo el tiempo y manejando errores de forma clara
    let start_load = Instant::now();
    let obj = match load_obj(&obj_path) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Error al cargar el archivo '{}': {}", obj_path, err);
            process::exit(1);
        }
    };
    let load_time = start_load.elapsed();

    // 3. Validación de índices dentro de rango
    if let Some(&max_idx) = obj.indices.iter().max() {
        assert!(
            max_idx < obj.vertices.len(),
            "Error: índice fuera de rango (índice {} para {} vértices)",
            max_idx,
            obj.vertices.len()
        );
    }

    // 4. Resumen corto y legible
    println!("Modelo cargado: '{}'", obj_path);
    println!("  Vértices:   {}", obj.vertices.len());
    println!("  Triángulos: {}", obj.indices.len() / 3);
    println!("  Tiempo:     {:?}", load_time);

    // 5. Configurar framebuffer 800x600 y calcular escala/offset automáticos (margin 10%)
    let width = 800;
    let height = 600;
    let (scale, offset) = fit_to_screen(&obj, width, height, 0.1);

    let mut framebuffer = Framebuffer::new(width, height, Color::new(15, 18, 28, 255));
    framebuffer.clear();

    // 6. Renderizar y exportar
    render_obj(&mut framebuffer, &obj, scale, offset);

    let output_file = "render.png";
    framebuffer.render_to_file(output_file);
    println!("Imagen exportada exitosamente a '{}'", output_file);
}
