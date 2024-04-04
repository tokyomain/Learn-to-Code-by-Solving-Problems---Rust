// CCC '07 J1 - Who is in the Middle?
// Problem Description: https://dmoj.ca/problem/ccc07j1
use std::io;

fn main() {
    let mut bowls: Vec<i32> = Vec::new();
    
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let parsed_input = input.trim().parse().expect("Error al convertir entero");
        bowls.push(parsed_input);
    }
    bowls.sort();

    println!("{}", bowls[1])
}
