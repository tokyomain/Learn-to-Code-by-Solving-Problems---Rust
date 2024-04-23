// COCI '18 Contest 4 #1 Elder
// Problem Description: https://dmoj.ca/problem/coci18c4p1

// Input 1: letter > wizzar the wand obey at start
// Input 2: int N (1<= N <= 100) > number of duels
// Input N: Z1 - Z2 > wizzard Z1 defeats wizzard Z2 
// Output 1: Which wizard did the wand obey after all N duels?
// Output 2: How many different wizards did the wand obey?
use std::io::stdin;


fn main() {
    // Creo variable para albergar a que mago obedece la varita
    let mut wand_obeys = String::new();
    stdin().read_line(&mut wand_obeys).expect("Error al leer la entrada");
    let mut wand_obeys = wand_obeys.trim().to_string();

    // Creo variable para albergar la cantidad de duelos
    let mut num_duelos = String::new();
    stdin().read_line(&mut num_duelos).expect("Error al leer la entrada");
    let num_duelos: usize = num_duelos.trim().parse().expect("Error al convertir en entero");
    
    // Creo vector para albergar datos de los distintos duelos
    let mut owners = vec![wand_obeys.clone()];

    // Creo loop para leer los datos de los duelos y albergarlos en el vector 'duelos'
    for _ in 0..num_duelos {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error al leer la entrada");
        let duelo_input: Vec<&str> = input.trim().split_whitespace().collect();
        
        if duelo_input[1] == wand_obeys {
            wand_obeys.clear();
            wand_obeys.push_str(duelo_input[0]);
            if !owners.contains(&wand_obeys) {
                owners.push(wand_obeys.clone())
            }
        }
    }
    println!("{}", wand_obeys);
    println!("{}", owners.len())
}
