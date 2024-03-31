use std::io;

// C = 5/9 x (F-32)  >>>  F = (C / 5/9) + 32
// Recibe grados gentigrados, devuelve grados Farenheit 
// Input: i32 = C
// Output: i32 = F


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let c: i32 = input.trim().parse().expect("Error al convertir en entero");
    
    let farenheit = (c * 9/5) + 32;

    println!("{:?}", farenheit)

}
