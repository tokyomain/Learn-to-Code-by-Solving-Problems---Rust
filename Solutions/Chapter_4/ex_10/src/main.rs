// COCI '08 Contest 3 #2 Kemija
// Problem Description: https://dmoj.ca/problem/coci08c3p2
use std::io;


fn main() {
    // Leo y almaceno input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada.");    

    // Creo estructura donde guardar los datos procesados
    let mut phrase = String::new();
    let mut skip_count = 0;

    for ch in input.chars() {
        if skip_count > 0 {
            skip_count -= 1;
        } else if "aeiou".contains(ch) {
            phrase.push(ch);
            skip_count = 2;
        } else {
            phrase.push(ch);
        }    
    } 
    println!("{phrase}")
}