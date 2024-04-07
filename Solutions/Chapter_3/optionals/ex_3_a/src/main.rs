// WC '17 Contest 3 J3 - Uncrackable
// Proble Description: https://dmoj.ca/problem/wc17c3j3
use std::io::stdin;

// 1. string between 8 and 12 characters long (inclusive)
// 2. lowercase letter (a … z), uppercase letter (A … Z), or digit (0 … 9)
//  2.a at least 3 lowercase letters, 
//  2.b at least 2 uppercase letters, 
//  2.c and at least one digit.

// Input: String > The password
// Output: Valid - Invalid

fn main() {
    // TODO: Leer input
    let mut pass = String::new();
    stdin().read_line(&mut pass).expect("Error al leer la entrada");
    let pass = pass.trim();
    
    // Chequeo len
    let is_valid_len = pass.len() >= 8 && pass.len() <= 12;

    // TODO: Inicializo contadores
    let mut upper_count = 0;
    let mut lower_count = 0;
    let mut digit_count = 0;

    // TODO: Chequear condiciones
   for character in pass.chars() {
        if character.is_ascii_uppercase() {
            upper_count += 1;
        } else if character.is_ascii_lowercase() {
            lower_count += 1;
        } else if character.is_ascii_digit() {
            digit_count += 1;
        } else {
            println!("Invalid");
            return;
        }

}
    let is_valid_times = upper_count >= 2 && lower_count >= 3 && digit_count >= 1;
    
    // Output
    if is_valid_len && is_valid_times {
        println!("Valid");
    } else {
        println!("Invalid");
    }
    //println!("Counters (up)(low)(dig): {}.{}.{}", upper_count, lower_count, digit_count);
}
