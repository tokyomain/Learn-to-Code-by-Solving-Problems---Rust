use std::io;


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la linea");

    let result: Vec<&str> = input.split_whitespace().collect();
    
    println!("{}", result.len());
}
