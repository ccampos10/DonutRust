use std::f32::consts::PI;

use std::thread;
use std::time::Duration;
use std::time::Instant;

mod render;
mod objetos;
use objetos::*;
use render::*;

// constantes globales
const RESOLUCION_VISTA_X: usize = 90; // resolucion de la vista. columnas
const RESOLUCION_VISTA_Y: usize = 45 *2; // resolucion de la vista. filas
const ORIGEN_2D: Vector2<u32> = Vector2{x: RESOLUCION_VISTA_X as u32/2, y: RESOLUCION_VISTA_Y as u32/2};  // ubicacion del origen en la vista
const ORIGEN_CAMARA: Vector3<f32> = Vector3{x: 0.0, y: 0.0, z: 330.0}; // ubicacion de la camara en el espacio 3d
const ORIGEN_LUZ: Vector3<f32> = Vector3{x: -5.0, y: -10.0, z: -1.0}; // direccion hacia donde apunta la luz global, no es la pocision de la fuente de luz.
const Z_PROYECCION: f32 = 100.0; // ubicacion del plano de proyeccion (el plano en donde se proyecta los puntos 3d a 2d para mostrarlos en la vista)
type Matriz = [[(u8,f32); RESOLUCION_VISTA_Y]; RESOLUCION_VISTA_X]; // tipo de la matriz de la vista

const DIMENCION: u32 = 60; // dimencion del cuadrado renderizado
const RADIO_DONA: u32 = 30;
const RADIO_ARANDELA: u32 = 15;

fn grad_to_rad(grad: i32) -> f32 {(grad as f32) * PI / 180.0}

fn main() {
    clear_terminal();
    // Renderizar un rectangulo en ascii
    // while 1>0 {
    //     for angulo_grad in 0..360{
    //         let matriz: Matriz = objetos::matriz_cuadrado(DIMENCION, grad_to_rad(angulo_grad), true);
    //         render_ascii(matriz);
    //         thread::sleep(Duration::from_millis(5));
    //     }
    // }

    // Renderizar un rectangulo en escala de grises
    // while 1>0 {
    //     for angulo_grad in 0..360{
    //         let matriz: Matriz = objetos::matriz_cuadrado(DIMENCION, grad_to_rad(angulo_grad), false);
    //         render_color(matriz);
    //         thread::sleep(Duration::from_millis(5));
    //     }
    // }

    // Renderizar una dona en ascii
    while 1>0 {
        for angulo_grad in 0..180{
            let inicio = Instant::now();
            let matriz: Matriz = objetos::matriz_dona(RADIO_DONA, RADIO_ARANDELA, grad_to_rad(angulo_grad*2), true);
            let duracion1 = inicio.elapsed();
            render_ascii(matriz);
            let duracion2 = inicio.elapsed();
            print!("duracion: {}{} - {} ", if duracion1.as_millis()<10  {"0"} else {""}, duracion1.as_millis(), duracion2.as_millis());
            // thread::sleep(Duration::from_millis(5));
        }
    }

    // Renderizar una dona en escala de grises
    // while 1>0 {
    //     for angulo_grad in 0..180{
    //         let inicio = Instant::now();
    //         let matriz: Matriz = objetos::matriz_dona(RADIO_DONA, RADIO_ARANDELA, grad_to_rad(angulo_grad*2), false);
    //         let duracion1 = inicio.elapsed();
    //         render_color(matriz);
    //         let duracion2 = inicio.elapsed();
    //         print!("duracion: {}{} - {} ", if duracion1.as_millis()<10  {"0"} else {""}, duracion1.as_millis(), duracion2.as_millis());
    //         // thread::sleep(Duration::from_millis(5));
    //     }
    // }
    
}
