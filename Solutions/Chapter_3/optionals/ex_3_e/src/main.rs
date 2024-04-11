// COCI '12 Contest 5 #1 Ljestvica
// Problem Description: https://dmoj.ca/problem/coci12c5p1

use std::io;

fn main() {
    // Lectura de input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    // Separar las notas en medidas
    let medida: Vec<&str> = input.trim().split('|').collect();

    // Crear contadores
    let mut contador_a = 0;
    let mut contador_c = 0;

    for medida in &medida {
        if let Some(primer_nota) = medida.trim().chars().next() {
            match primer_nota {
                'A' | 'D' | 'E' => contador_a += 1,
                'C' | 'F' | 'G' => contador_c += 1,
                _ => (),
            }
        }
    }

    // Determinar la escala
    let escala = if contador_a > contador_c {
        "A-mol"
    } else if contador_c > contador_a {
        "C-dur"
    } else {
        // Si hay igual cantidad de notas principales, decidir basado en la ultima nota
        match input.trim().chars().rev().next() {
            Some('A') => "A-mol",
            _ => "C-dur",
        }
    };

    // Imprimir resultado
    println!("{}", escala);
}