use crate::*;

// const CHARACTER1: char = '@'; // caracter del objeto
// const CHARACTER2: char = ' '; // caracter del fondo
const CHARACTERS: [char; 32] = [' ','`','.','\'',',',':',';','=','+','*','/','i','|','v','c','o','Z','A','K','U','H','Q','0','W','N','$','&','O','8','%','#','@'];

pub fn clear_terminal(){
    print!("\x1b[2J\x1b[H\x1b[?25l");
}

fn move_cursor(){
    print!("\x1b[H");
}

pub fn render(matriz: Matriz){
    move_cursor();
    // por como funciona el imprimir cosas por pantalla, primero me muevo en y luego en x
    // por cada linea
    for y in (0..RESOLUCION_Y).rev(){
        // por cada caracter
        for x in 0..RESOLUCION_X{
            print!("{}",CHARACTERS[matriz[x][y] as usize]);
        }
        print!("\n");
    }
}