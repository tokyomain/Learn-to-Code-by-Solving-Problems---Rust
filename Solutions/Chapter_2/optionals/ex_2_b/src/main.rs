// CCC '15 J1 - Special Day
// Problem description: https://dmoj.ca/problem/ccc15j1

use std::io;

// Input: 2 Lines > 2 Ints > 1st: Month 2ns: Day
// Output: Before - After - Special



fn main() {
    let mut input_vector: Vec<i32> = Vec::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let number1: i32 = input.trim().parse().expect("Error al convertir en entero");
    input_vector.push(number1);

    input.clear();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let number2: i32 = input.trim().parse().expect("Error al convertir en entero");
    input_vector.push(number2);

    match input_vector[0] {
        1 => println!("Before"),
        2 => match input_vector[1] {
            x if x < 18 => println!("Before"),
            18 => println!("Special"),
            _ => println!("After"),
        }
        _ => println!("After"),
    }
}
