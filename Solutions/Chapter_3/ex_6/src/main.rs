// CCC '18 J2 - Occupy parking
// Problem Description: https://dmoj.ca/problem/ccc18j2
use std::io;

fn main() {
    let mut count = 0;

    let mut parking_lots = String::new();
    io::stdin().read_line(&mut parking_lots).expect("Error al leer la entrada");

    let mut yesterday = String::new();
    io::stdin().read_line(&mut yesterday).expect("Error al leer la entrada");
    let yesterday = yesterday.trim();

    let mut today = String::new();
    io::stdin().read_line(&mut today).expect("Error al leer la entrada");
    let today = today.trim();

    if yesterday.len() != today.len() {
        panic!("Las cadenas de entrada deben tener la misma longitud.");
    }
    
    let chars_yesterday = yesterday.chars();
    let chars_today = today.chars();

    for (c1, c2) in chars_yesterday.zip(chars_today) {
        if c1 == 'C' &&  c2 == 'C' {
            count += 1;
        }  
    }
    println!("{}", count);
}
