use std::f64::consts::PI;

use std::thread;
use std::time::Duration;

mod render;
mod objetos;
use render::{render, clear_terminal};

// constantes globales
const RESOLUCION_X: usize = 192;
const RESOLUCION_Y: usize = 64;
type Matriz = [[u8; RESOLUCION_Y]; RESOLUCION_X];
const ORIGEN_2D: (i16, i16) = (RESOLUCION_X as i16/2,RESOLUCION_Y as i16/2);
const ORIGEN_CAMARA: (i16, i16, i16) = (0, 0, 330);
const Z_PROYECCION: i16 = 100;

const DIMENCION: u8 = 128;

fn main() {
    clear_terminal();
    while true {
        for angulo_grad in 0..360{
            let matriz: Matriz = objetos::matriz_plano(DIMENCION, (angulo_grad as f64) * PI / 180.0);
            render(matriz);
            thread::sleep(Duration::from_millis(5));
        }
    }
}
