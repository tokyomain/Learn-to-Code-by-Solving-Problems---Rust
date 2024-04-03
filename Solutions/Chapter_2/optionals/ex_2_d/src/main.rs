// DMOPC '16 Contest 1 P0 - C.C. and Cheese-kun
// Problem description: https://dmoj.ca/problem/dmopc16c1p0
// Output: C.C. is M satisfied with her pizza.
use std::io;

fn determine_satisfaction(width_input: i32, cheesiness_input: i32) -> &'static str {
    if width_input == 3 && cheesiness_input >= 95 {
        "absolutely"
    } else if width_input == 1 && cheesiness_input <= 50 {
        "fairly"
    } else {
        "very"
    }
}

fn main() {
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).expect("Error al leer la entrada");
    let width_input = width_input.trim().parse().expect("Error al convertir en entero");

    let mut cheesiness_input = String::new();
    io::stdin().read_line(&mut cheesiness_input).expect("Error al leer la entrada");
    let cheesiness_input = cheesiness_input.trim().parse().expect("Error al convertir en entero");
    
    let satisfaction = determine_satisfaction(width_input, cheesiness_input);

    println!("C.C. is {} satisfied with her pizza.", satisfaction);
}
