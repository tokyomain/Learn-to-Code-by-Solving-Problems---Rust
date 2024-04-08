// COCI '18 Contest 3 #1 Magnus
// Problem Description: https://dmoj.ca/problem/coci18c3p1

// Input: Word of len = N ( 1 ≤ N ≤ 100 000 ) 



// Iterar sobre word > buscar H > O > N > I
// Llevar cuenta de cuantos bloques HONI
// Output: i32 > cuenta

use std::io::stdin;

fn main() {
    // Read Input
    let mut word = String::new();
    stdin().read_line(&mut word).expect("Error al leer la entrada");
    // Define HONI block
    let block = "HONI";
    // Define counter
    let mut counter = 0;
    // Define index
    let mut index = 0;   

    for ch in word.chars() {
        if ch == block.chars().nth(index).unwrap() {
            index += 1;
            if index == block.len() {
                counter += 1;
                index = 0;
            }
        }
    }
    println!("{}", counter)
}
