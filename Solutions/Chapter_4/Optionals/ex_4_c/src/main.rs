// COCI '08 Contest 1 #2 Ptice
// Problem Description: https://dmoj.ca/problem/coci08c1p2
use std::io;

fn main() {
    // Leo input 1: i32  > Len de secuencia correcta
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("Error al leer la entrada.");
    length = length.trim().parse().expect("Error al convertir en entero.");
    // Leo input 2: Str > Secuencia correcta
    let mut correct_seq = String::new();
    io::stdin().read_line(&mut correct_seq).expect("Error al leer el input.");
    // Creo array para nombres    
    let array:[&str;3] = ["Adrian", "Bruno", "Goran"];
    // Creo vector para las 3 sequencias (esto podria ser un array, ver)
    let sequences = vec!["ABC", "BABC", "CCAABB"];
    // Creo contador para llevar la mayor cantidad de respuestas correctas
    let mut correct_ans = 0;
    // Creo iterador sobre la sequencia correcta
    let iter_correct = correct_seq.chars();
    // Creo vector para llevar nombres con mayor respuestas correctas
    let mut names = Vec::new();
    // Creo variable para iterar
    let mut index = 0;
    
    for seq in sequences {
        
        let mut correct_temp = 0;
        // Utilizo metodo cycle sobre cada una de las sequencias
        let iter_play = seq.chars().cycle();
        
        let zipped = iter_play.zip(iter_correct.clone());
        for (item1, item2) in zipped {
            if item1 == item2 {
                correct_temp += 1;
            }
        }
        if correct_temp > correct_ans {
            names.clear();
            names.push(array[index]);
            correct_ans = correct_temp;
        } else if correct_temp == correct_ans {
            names.push(array[index])
        }
        index += 1;
    }
    println!("{}", correct_ans);
    for e in names.iter(){
        println!("{e}");   
    }
}