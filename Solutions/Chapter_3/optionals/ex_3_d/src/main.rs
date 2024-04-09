// CCC '11 S2 - Multiple Choice
// Problem Description: https://dmoj.ca/problem/ccc11s2

// Input:   - int >  N ( 0 < N < 10 000 ) 
//          - 2 N lines:
//              -  N lines of student responses
//              -  N lines of correct answers
// Output:  - intC ( 0 ≤ C ≤ N ) number of correct answers.

use std::io;


fn main() {
    // Vector para almacenar las respuestas de los estudiantes
    let mut student_ans: Vec<String> = Vec::new();
     // Vector para almacenar las respuestas correctas
    let mut correct_ans: Vec<String> = Vec::new();

    // Lectura del número de preguntas
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let input = input.trim().parse().expect("Error al convertir en entero");

    // Lectura de las respuestas de los estudiantes
    for _line in 0..input {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let input = input.trim().to_string();
        student_ans.push(input);
    }
    // Lectura de las respuestas correctas
    for _line in 0..input {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let input = input.trim().to_string();
        correct_ans.push(input)
    }
    // Combinar los iteradores de respuestas de estudiantes y respuestas correctas
    let iterador_zip = student_ans.iter().zip(correct_ans.iter());
    // Incializar contador de respuestas correctas
    let mut counter = 0;
    // Contar el número de respuestas correctas
    for (char1, char2) in iterador_zip {
        if char1 == char2 {
            counter += 1;
        }
    }
    // Imprimir el número de respuestas correctas
    println!("{counter}")
}
