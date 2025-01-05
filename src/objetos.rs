use crate::*;

const ORIGEN_FIGURA: (i16, i16, i16) = (DIMENCION as i16/2,DIMENCION as i16/2,0);

fn map_range(value: i16, from_min: i16, from_max: i16, to_min: i16, to_max: i16) -> i16 {
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

pub fn matriz_plano(dimencion: u8, angulo: f64/*recivir la matriz de rotacion*/) -> Matriz{
    // let eje_rot: [f64; 3] = [1.0/(2_f64.sqrt()),1.0/(2_f64.sqrt()),0.0];
    // let eje_rot: [f64; 3] = [2.0/(5_f64.sqrt()),1.0/(5_f64.sqrt()),0.0];
    let eje_rot: [f64; 3] = [1.0/(3_f64.sqrt()),1.0/(3_f64.sqrt()),1.0/(3_f64.sqrt())];
    let matriz_trans: [[f64; 3]; 3] = [ [angulo.cos()+eje_rot[0].powi(2)*(1.0-angulo.cos()),                  eje_rot[0]*eje_rot[1]*(1.0-angulo.cos())-eje_rot[2]*angulo.sin(),     eje_rot[0]*eje_rot[2]*(1.0-angulo.cos())+eje_rot[1]*angulo.sin()], // matriz de la transformacion (en este caso rotacion)
                                        [eje_rot[0]*eje_rot[1]*(1.0-angulo.cos())+eje_rot[2]*angulo.sin(),    angulo.cos()+eje_rot[1].powi(2)*(1.0-angulo.cos()),                   eje_rot[1]*eje_rot[2]*(1.0-angulo.cos())-eje_rot[0]*angulo.sin()],
                                        [eje_rot[2]*eje_rot[0]*(1.0-angulo.cos())-eje_rot[1]*angulo.sin(),    eje_rot[2]*eje_rot[1]*(1.0-angulo.cos())+eje_rot[0]*angulo.sin(),     angulo.cos()+eje_rot[2].powi(2)*(1.0-angulo.cos())]];

    let matriz_cuadrador: [[f64; 3]; 3] = [ [1.0,   0.0,    0.0],
                                            [0.0,   0.5,    0.0],
                                            [0.0,   0.0,    1.0]];

    let mut matriz: Matriz = [[0; RESOLUCION_Y]; RESOLUCION_X];
    for x in 0..dimencion{
        for y in 0..dimencion{
            // let z = ((VECTOR_NORMAL.0*(ORIGEN_FIGURA.0-x)+VECTOR_NORMAL.1*(ORIGEN_FIGURA.1-y))/VECTOR_NORMAL.2)+ORIGEN_FIGURA.2;
            let z = 0;
            let mut x_trans = ((x as f64 - ORIGEN_FIGURA.0 as f64)*matriz_trans[0][0]+(y as f64 - ORIGEN_FIGURA.1 as f64)*matriz_trans[0][1]+(z as f64 - ORIGEN_FIGURA.2 as f64)*matriz_trans[0][2]) as i16;
            let mut y_trans = ((x as f64 - ORIGEN_FIGURA.0 as f64)*matriz_trans[1][0]+(y as f64 - ORIGEN_FIGURA.1 as f64)*matriz_trans[1][1]+(z as f64 - ORIGEN_FIGURA.2 as f64)*matriz_trans[1][2]) as i16;
            let mut z_trans = ((x as f64 - ORIGEN_FIGURA.0 as f64)*matriz_trans[2][0]+(y as f64 - ORIGEN_FIGURA.1 as f64)*matriz_trans[2][1]+(z as f64 - ORIGEN_FIGURA.2 as f64)*matriz_trans[2][2]) as i16;

            // aplicar perspectiva
            let escalar: f32 = (ORIGEN_CAMARA.2-z_trans) as f32/(ORIGEN_CAMARA.2-Z_PROYECCION) as f32;
            x_trans = (x_trans as f32/escalar) as i16;
            y_trans = (y_trans as f32/escalar) as i16;

            // ajustar a la terminal
            x_trans = ((x_trans as f64)*matriz_cuadrador[0][0]+(y_trans as f64)*matriz_cuadrador[0][1]+(z_trans as f64)*matriz_cuadrador[0][2]) as i16;
            y_trans = ((x_trans as f64)*matriz_cuadrador[1][0]+(y_trans as f64)*matriz_cuadrador[1][1]+(z_trans as f64)*matriz_cuadrador[1][2]) as i16;
            z_trans = ((x_trans as f64)*matriz_cuadrador[2][0]+(y_trans as f64)*matriz_cuadrador[2][1]+(z_trans as f64)*matriz_cuadrador[2][2]) as i16;

            matriz[(x_trans+ORIGEN_2D.0) as usize][(y_trans+ORIGEN_2D.1) as usize] = map_range(z_trans, -90, 90, 0, 31) as u8;
        }
    }
    return matriz;
}