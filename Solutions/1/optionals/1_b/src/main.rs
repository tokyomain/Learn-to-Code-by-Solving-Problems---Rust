use std::io;

// A long time ago in a galaxy far, far away...
// N ( 1 ≤ N ≤ 5 ) 
// Input: (Integer) 3
// Output:  A long time ago in a galaxy far, far, far, far away...

fn main() {
    let part_1 = String::from("A long time ago in a galaxy ");
    let part_3 = String::from("far away...");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let cantidad: usize = input.trim().parse().expect("Error al convertir en entero");

    let part_2: String = vec!["far, "; cantidad - 1].join("");

    let frase_completa = part_1 + &part_2 + &part_3;

    println!("{frase_completa}");
        
}