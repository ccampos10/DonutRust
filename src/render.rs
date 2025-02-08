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

pub fn render_ascii(matriz: Matriz){
    move_cursor();
    // por como funciona el imprimir cosas por pantalla, primero me muevo en y luego en x
    // por cada linea
    for y in (0..RESOLUCION_VISTA_Y/2){
        // por cada caracter
        for x in 0..RESOLUCION_VISTA_X{
            print!("{}",CHARACTERS[matriz[x][y].0 as usize ]);
        }
        print!("\n");
    }
}

pub fn render_color(matriz: Matriz){
    move_cursor();
    for y in 0..(RESOLUCION_VISTA_Y/2){ // puede ser par o impar
        for x in 0..RESOLUCION_VISTA_X{
            let char;
            if matriz[x][y*2].0 != 0 && matriz[x][y*2+1].0 != 0{
                char = '█';
            }
            else if matriz[x][y*2].0 != 0 && matriz[x][y*2+1].0 == 0 {
                char = '▀';
            }
            else if matriz[x][y*2].0 == 0 && matriz[x][y*2+1].0 != 0 {
                char = '▄';
            }
            else { char = ' '; }

            let color: u8;
            if matriz[x][y*2].0 == 0 || matriz[x][y*2+1].0 == 0 { color = 232 + (matriz[x][y*2].0+matriz[x][y*2+1].0); }
            else { color = 232 + (matriz[x][y*2].0+matriz[x][y*2+1].0)/2; }
            print!("\x1b[38;5;{}m{}\x1b[0m",color,char);
            // print!("{}",char);
            // print!("{}",CHARACTERS[matriz[x][y].0 as usize ]);
        }
        print!("\n");
        // si es impar se esta perdiendo una linea
    }
}
