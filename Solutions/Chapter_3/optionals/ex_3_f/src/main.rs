// COCI '13 Contest 3 #1 Riječi
// Problem Description: https://dmoj.ca/problem/coci13c3p1

// Input: int >  K ( 1 ≤ K ≤ 45 ) Number of times the button was pressed
// Output:  two space-separated ints > Number of letters As and Bs

// B get transformed to BA
// A get transformed to B

use std::io;

fn main() {
    // Crear variable para almacenar el input
    let mut input = String::new();

    // Metodo 'read_line()' retorna un Result que puede ser Ok (exito) o Err (error)
    // Se utiliza 'expect()' para mostrar el mensaje especificado en caso de error
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    
    // Convierte el valor del input en un i32
    let input: i32 = input.trim().parse().expect("Error al convertir en entero");

    // Inicializa dos variables mutables con las cuales realizar las operaciones necesarias
    let (mut n1, mut n2) = (0, 1);
    
    // Crea variable mutable, la suma de las dos variables anteriores n1 y n2
    let mut sum = n1 + n2;

    // Itera sobre un rango segun input y realiza operaciones acordes
    for _num in 0..(input - 1) {
        n1 = n2;
        n2 = sum;
        sum = n1 + n2
    }

    // Imprime output
    println!("{n1} {n2}")

}
