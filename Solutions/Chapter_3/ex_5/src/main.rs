// COCI '06 Contest 5 #1 Trik
// Problem Description: https://dmoj.ca/problem/coci06c5p1
use std::io;

fn main() {
    let mut cups = vec![1, 0, 0];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    
    for action in input.trim().chars() {
        match action {
            'A' => { cups.swap(0, 1);}
            'B' => { cups.swap(1,2);}
            'C' => { cups.swap(0,2);}
            _ => println!("Accion no valida")
        }
    }
    if let Some(index) = cups.iter().position(|&x| x == 1) {
        println!("{}", index+1);
    } else {
        println!("No se encontro la bola.");
    }
}
