use std::ops::{Add, Mul};

use crate::*;

pub struct Vector3<T> { pub x: T, pub y: T, pub z: T }

impl <T> Vector3<T> where T: Mul<Output = T> + Add<Output = T> + Copy {
    pub fn ppunto(&self, other: &Vector3<T>) -> T { self.x*other.x + self.y*other.y + self.z*other.z }
}
impl Vector3<f32>{
    pub fn modulo(&self) -> f32 { (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt() }
}

pub struct Vector2<T> { pub x: T, pub y: T }

fn map_range(value: i16, from_min: i16, from_max: i16, to_min: i16, to_max: i16) -> i16 {
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

pub fn matriz_cuadrado(dimencion: u32, angulo: f32, para_ascii: bool) -> Matriz{
    let origen_figura: Vector3<f32> = Vector3{x: dimencion as f32/2.0, y: dimencion as f32/2.0, z: 0.0}; // calcular el origen de la figura (centro)
    let mut vector_normal_plano: Vector3<f32> = Vector3{x: 0.0, y: 0.0, z: 1.0};

    // let eje_rot: Vector3<f32> = Vector3{x: 0.0, y: 0.0, z: 1.0};
    // let eje_rot: Vector3<f32> = Vector3{x: 1.0/(2_f32.sqrt()), y: 1.0/(2_f32.sqrt()), z: 0.0};
    let eje_rot: Vector3<f32> = Vector3{x: 1.0/(3_f32.sqrt()), y: 1.0/(3_f32.sqrt()), z: 1.0/(3_f32.sqrt())};   
    let matriz_trans: [[f32; 3]; 3] = [ [angulo.cos()+eje_rot.x.powi(2)*(1.0-angulo.cos()),                  eje_rot.x*eje_rot.y*(1.0-angulo.cos())-eje_rot.z*angulo.sin(),     eje_rot.x*eje_rot.z*(1.0-angulo.cos())+eje_rot.y*angulo.sin()], // matriz de la transformacion (en este caso rotacion)
                                        [eje_rot.x*eje_rot.y*(1.0-angulo.cos())+eje_rot.z*angulo.sin(),    angulo.cos()+eje_rot.y.powi(2)*(1.0-angulo.cos()),                   eje_rot.y*eje_rot.z*(1.0-angulo.cos())-eje_rot.x*angulo.sin()],
                                        [eje_rot.z*eje_rot.x*(1.0-angulo.cos())-eje_rot.y*angulo.sin(),    eje_rot.z*eje_rot.y*(1.0-angulo.cos())+eje_rot.x*angulo.sin(),     angulo.cos()+eje_rot.z.powi(2)*(1.0-angulo.cos())]];

    let matriz_cuadrador: [[f32; 3]; 3] = [ [1.0,   0.0,    0.0], // matriz que ajusta el render a la terminal
                                            [0.0,   0.5,    0.0],
                                            [0.0,   0.0,    1.0]];

    let mut matriz: Matriz = [[(0, 0.0); RESOLUCION_VISTA_Y]; RESOLUCION_VISTA_X];

    // calcular sombra
    vector_normal_plano.x = vector_normal_plano.x*matriz_trans[0][0] + vector_normal_plano.y*matriz_trans[0][1] + vector_normal_plano.z*matriz_trans[0][2];
    vector_normal_plano.y = vector_normal_plano.x*matriz_trans[1][0] + vector_normal_plano.y*matriz_trans[1][1] + vector_normal_plano.z*matriz_trans[1][2];
    vector_normal_plano.z = vector_normal_plano.x*matriz_trans[2][0] + vector_normal_plano.y*matriz_trans[2][1] + vector_normal_plano.z*matriz_trans[2][2];
    let angulo_sombra = (vector_normal_plano.ppunto(&ORIGEN_LUZ)/(vector_normal_plano.modulo()*ORIGEN_LUZ.modulo())).acos() * 180.0 / PI;


    for x in 0..dimencion{
        for y in 0..dimencion{
            let z = 0;
            // let punto = (x,y,0);

            let mut punto_new: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
            punto_new.x = (x as f32 - origen_figura.x)*matriz_trans[0][0]+(y as f32 - origen_figura.y)*matriz_trans[0][1]+(z as f32 - origen_figura.z)*matriz_trans[0][2];
            punto_new.y = (x as f32 - origen_figura.x)*matriz_trans[1][0]+(y as f32 - origen_figura.y)*matriz_trans[1][1]+(z as f32 - origen_figura.z)*matriz_trans[1][2];
            punto_new.z = (x as f32 - origen_figura.x)*matriz_trans[2][0]+(y as f32 - origen_figura.y)*matriz_trans[2][1]+(z as f32 - origen_figura.z)*matriz_trans[2][2];

            // aplicar perspectiva
            let escalar: f32 = (ORIGEN_CAMARA.z-punto_new.z).abs()/(ORIGEN_CAMARA.z-Z_PROYECCION).abs();
            punto_new.x = ((ORIGEN_CAMARA.x-punto_new.x)/escalar)+ORIGEN_CAMARA.x;
            punto_new.y = ((ORIGEN_CAMARA.y-punto_new.y)/escalar)+ORIGEN_CAMARA.y;

            // ajustar a la terminal
            if para_ascii{
                punto_new.x = punto_new.x*matriz_cuadrador[0][0] + (punto_new.y)*matriz_cuadrador[0][1] + (punto_new.z)*matriz_cuadrador[0][2];
                punto_new.y = punto_new.x*matriz_cuadrador[1][0] + (punto_new.y)*matriz_cuadrador[1][1] + (punto_new.z)*matriz_cuadrador[1][2];
                punto_new.z = punto_new.x*matriz_cuadrador[2][0] + (punto_new.y)*matriz_cuadrador[2][1] + (punto_new.z)*matriz_cuadrador[2][2];
            }

            let mod_origen: u32;
            let max_value: i16;
            if para_ascii {max_value = 31; mod_origen=2;}
            else {max_value = 23; mod_origen=1;}

            if (punto_new.x+ORIGEN_2D.x as f32) as usize >= RESOLUCION_VISTA_X || (punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize >= RESOLUCION_VISTA_Y { continue; }

            matriz[(punto_new.x+ORIGEN_2D.x as f32) as usize][(punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize].0 = map_range(angulo_sombra as i16, 0, 180, 0, max_value) as u8;
        }
    }
    return matriz;
}

pub fn matriz_dona(r1: u32, r2: u32, angulo: f32, para_ascii: bool) -> Matriz{
    let cos_angulo = angulo.cos();
    let sin_angulo = angulo.sin();

    // let eje_rot: Vector3<f32> = Vector3{x: 0.0, y: 0.0, z: 1.0};
    // let eje_rot: Vector3<f32> = Vector3{x: 1.0/(2_f32.sqrt()), y: 1.0/(2_f32.sqrt()), z: 0.0};
    let eje_rot: Vector3<f32> = Vector3{x: 1.0/(3_f32.sqrt()), y: 1.0/(3_f32.sqrt()), z: 1.0/(3_f32.sqrt())};
    let matriz_trans: [[f32; 3]; 3] = [ [cos_angulo+eje_rot.x.powi(2)*(1.0-cos_angulo),                  eje_rot.x*eje_rot.y*(1.0-cos_angulo)-eje_rot.z*sin_angulo,     eje_rot.x*eje_rot.z*(1.0-cos_angulo)+eje_rot.y*sin_angulo], // matriz de la transformacion (en este caso rotacion)
                                        [eje_rot.x*eje_rot.y*(1.0-cos_angulo)+eje_rot.z*sin_angulo,    cos_angulo+eje_rot.y.powi(2)*(1.0-cos_angulo),                   eje_rot.y*eje_rot.z*(1.0-cos_angulo)-eje_rot.x*sin_angulo],
                                        [eje_rot.z*eje_rot.x*(1.0-cos_angulo)-eje_rot.y*sin_angulo,    eje_rot.z*eje_rot.y*(1.0-cos_angulo)+eje_rot.x*sin_angulo,     cos_angulo+eje_rot.z.powi(2)*(1.0-cos_angulo)]];

    let matriz_cuadrador: [[f32; 3]; 3] = [ [1.0,   0.0,    0.0], // matriz que ajusta el render a la terminal
                                            [0.0,   0.5,    0.0],
                                            [0.0,   0.0,    1.0]];

    let mut matriz: Matriz = [[(0, f32::MIN); RESOLUCION_VISTA_Y]; RESOLUCION_VISTA_X];

    for ang_dona in 0..360{
        for ang_arandela in 0..360{
            let cos_ang_arandela = grad_to_rad(ang_arandela).cos();
            let sin_ang_arandela = grad_to_rad(ang_arandela).sin();
            let cos_ang_dona = grad_to_rad(ang_dona).cos();
            let sin_ang_dona = grad_to_rad(ang_dona).sin();

            let mut punto: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

            punto.x = cos_ang_dona*((r1 as f32)+(r2 as f32)*cos_ang_arandela);
            punto.y = sin_ang_dona*((r1 as f32)+(r2 as f32)*cos_ang_arandela);
            punto.z = (r2 as f32)*sin_ang_arandela;

            let mut punto_new: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
            punto_new.x = (punto.x as f32)*matriz_trans[0][0] + (punto.y as f32)*matriz_trans[0][1] + (punto.z as f32)*matriz_trans[0][2];
            punto_new.y = (punto.x as f32)*matriz_trans[1][0] + (punto.y as f32)*matriz_trans[1][1] + (punto.z as f32)*matriz_trans[1][2];
            punto_new.z = (punto.x as f32)*matriz_trans[2][0] + (punto.y as f32)*matriz_trans[2][1] + (punto.z as f32)*matriz_trans[2][2];

            // aplicar perspectiva
            let escalar: f32 = (ORIGEN_CAMARA.z-punto_new.z).abs()/(ORIGEN_CAMARA.z-Z_PROYECCION).abs();
            punto_new.x = ((ORIGEN_CAMARA.x+punto_new.x)/escalar)+ORIGEN_CAMARA.x;
            punto_new.y = ((ORIGEN_CAMARA.y+punto_new.y)/escalar)+ORIGEN_CAMARA.y;

            // ajustar a la terminal
            if para_ascii {
                punto_new.x = punto_new.x*matriz_cuadrador[0][0] + (punto_new.y)*matriz_cuadrador[0][1] + (punto_new.z)*matriz_cuadrador[0][2];
                punto_new.y = punto_new.x*matriz_cuadrador[1][0] + (punto_new.y)*matriz_cuadrador[1][1] + (punto_new.z)*matriz_cuadrador[1][2];
                punto_new.z = punto_new.x*matriz_cuadrador[2][0] + (punto_new.y)*matriz_cuadrador[2][1] + (punto_new.z)*matriz_cuadrador[2][2];
            }

            let mod_origen: u32;
            let max_value: i16;
            if para_ascii {max_value = 31; mod_origen=2;}
            else {max_value = 23; mod_origen=1;}
            if (punto_new.x+ORIGEN_2D.x as f32) as usize >= RESOLUCION_VISTA_X || (punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize >= RESOLUCION_VISTA_Y { continue; }

            // calcular sombra
            let mut vector_normal: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
            vector_normal.x = cos_ang_arandela*cos_ang_dona;
            vector_normal.y = cos_ang_arandela*sin_ang_dona;
            vector_normal.z = sin_ang_arandela;

            let mut vector_normal_new: Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
            vector_normal_new.x = (vector_normal.x as f32)*matriz_trans[0][0] + (vector_normal.y as f32)*matriz_trans[0][1] + (vector_normal.z as f32)*matriz_trans[0][2];
            vector_normal_new.y = (vector_normal.x as f32)*matriz_trans[1][0] + (vector_normal.y as f32)*matriz_trans[1][1] + (vector_normal.z as f32)*matriz_trans[1][2];
            vector_normal_new.z = (vector_normal.x as f32)*matriz_trans[2][0] + (vector_normal.y as f32)*matriz_trans[2][1] + (vector_normal.z as f32)*matriz_trans[2][2];

            let angulo_sombra = (vector_normal_new.ppunto(&ORIGEN_LUZ)/(vector_normal_new.modulo()*ORIGEN_LUZ.modulo())).acos() * 180.0 / PI;

            let val_mapeado = map_range(angulo_sombra as i16, 0, 180, 0, max_value) as u8;
            if matriz[(punto_new.x+ORIGEN_2D.x as f32) as usize][(punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize].1 < punto_new.z{
                matriz[(punto_new.x+ORIGEN_2D.x as f32) as usize][(punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize].1 = punto_new.z;
                matriz[(punto_new.x+ORIGEN_2D.x as f32) as usize][(punto_new.y+(ORIGEN_2D.y/mod_origen) as f32) as usize].0 = val_mapeado;
            }

        }
    }
    return matriz;
}