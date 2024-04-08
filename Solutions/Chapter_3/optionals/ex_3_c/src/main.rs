// CCC '11 S1 - English or French?
// Problem Description: https://dmoj.ca/problem/ccc11s1

// Input: 1 > N  ( 0 < N < 10 000 ) 
//        2 > N lines 
// Output: English - French

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");
    let number: i32 = input.trim().parse().expect("Error al convertir en entero");
    
    let mut word = String::new();

    for _ in 0..number {
        let mut input2 = String::new();
        stdin().read_line(&mut input2).expect("Error al leer la entrada");
        word += &input2;
    }

    // Counters
    let mut s_count = 0;
    let mut t_count = 0;

    for ch in word.chars() {
        match ch {
            's' | 'S' => s_count += 1,
            't' | 'T' => t_count += 1,
            _ => {}
        }
    }

    if t_count > s_count {
        println!("English")
    } else {
        println!("French")
    }
    
}
