use std::io::{self, BufRead};


#[derive(Debug)]
struct Equipo {
    puntaje1: u32,
    puntaje2: u32,
    puntaje3: u32,
}
impl Equipo {
    // Metodo para operar sobre los puntajes
    fn multiplicar(&mut self) {
        self.puntaje1 *= 3;
        self.puntaje2 *= 2;
    }

    // Metodo para sumar los puntajes
    fn resultado(&mut self) -> u32 {
        self.puntaje1 + self.puntaje2 + self.puntaje3
    }
}

fn main() {
    // Creo un vector para almacenar los equipos
    let mut equipos: Vec<Equipo> = Vec::new();
    
    // Menejo de la entrada estandar
    let stdin = io::stdin(); // Este objeto es un "manejador"(handler) de la entrada estándar
    let mut lineas = stdin.lock().lines();

    // Leer los datos para cada equipo
    for _ in 0..2 {
        let puntaje1: u32 = lineas.next().unwrap().unwrap().trim().parse().expect("Error al leer puntaje1");
        let puntaje2: u32 = lineas.next().unwrap().unwrap().trim().parse().expect("Error al leer puntaje2");
        let puntaje3: u32 = lineas.next().unwrap().unwrap().trim().parse().expect("Error al leer puntaje3");


        // Crear una nueva instancia de Team y agregarla al vector
        let mut equipo = Equipo {
            puntaje1,
            puntaje2,
            puntaje3,
        };
        equipo.multiplicar();
        equipos.push(equipo);
    }

    // Calcular la suma total de los puntajes de cada equipo
    let resultado_equipo_a = equipos[0].resultado();
    let resultado_equipo_b = equipos[1].resultado();

    // Determino el ganador
    let ganador = if  resultado_equipo_a > resultado_equipo_b {
        "A"
    } else if resultado_equipo_a < resultado_equipo_b {
        "B"
    } else {
        "T"
    };
    
    println!("{}", ganador)

}
