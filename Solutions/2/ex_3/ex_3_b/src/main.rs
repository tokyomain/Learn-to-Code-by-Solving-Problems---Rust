use std::io;
use std::cmp::Ordering;

fn leer_entero() -> i32 {
    let mut entero = String::new();
    io::stdin().read_line(&mut entero).expect("Error al leer la linea");
    entero.trim().parse().expect("Se espera un numero")
}

fn main() {
    // Leer puntajes para el equipo A
    let puntaje_a = 
        (3 * leer_entero()) + 
        (2 * leer_entero()) + 
        leer_entero();

    // Leer puntajes para el equipo B
    let puntaje_b =
        (3 * leer_entero()) + 
        (2 * leer_entero()) + 
        leer_entero();
        
    // Determinar el resultado del juego
    match puntaje_a.cmp(&puntaje_b) {
        Ordering::Less => println!("B"),
        Ordering::Greater => println!("A"),
        Ordering::Equal => println!("T"),
    }
}
