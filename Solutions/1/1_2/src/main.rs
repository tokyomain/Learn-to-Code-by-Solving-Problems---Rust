use std::{f64::consts::PI, io};


// r: radius
// h: height
// V= pi.r**2.h / 3


fn main()  {
    
    let mut numbers: Vec<f64> = Vec::new();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input) // reads the line
        .expect("Error al leer la linea"); // handle error
    
    let number1: f64 = input
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input no es un número entero"); // handle error
    
    input.clear(); // Limpia el buffer de entrada para poder leer el siguiente input

    io::stdin()
        .read_line(&mut input) // reads the line
        .expect("Error al leer la linea"); // handle error
    
    let number2: f64 = input
        .trim() // ignore whitespaec around input
        .parse() // convert to integers
        .expect("Input no es un número entero"); // handle error

    numbers.push(number1);
    numbers.push(number2);

    let operation = 
    PI* numbers[0].powi(2)* numbers[1] / 3.0;

    println!("{}", operation)
}
