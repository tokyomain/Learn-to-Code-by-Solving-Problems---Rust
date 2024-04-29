// CCC '20 J2 - Epidemiology
// Problem Description: https://dmoj.ca/problem/ccc20j2

// Inputs:
//       1. P: Number to reach
//       2. N: Persons have the disease
//       3. R: Infections per person
// Output: i32 > Number of day to reach P infected people

use std::io::{self, stdin};


fn main() {
    let mut p: String = String::new();
    stdin().read_line(&mut p).expect("Error al leer la entrada.");
    let num_reach: i32 = p.trim().parse().expect("Error al convertir en entero.");

    let mut n = String::new();
    stdin().read_line(&mut n).expect("Error al leer la entrada.");
    let mut infected_people: i32 = n.trim().parse().expect("Error al convertir en entero.");

    let mut r = String::new();
    stdin().read_line(&mut r).expect("Error al leer la entrada.");
    let infections_per_person: i32 = r.trim().parse().expect("Error al convertir en entero.");

    let mut count_day: i32 = 0;
    let mut total_infected: i32 = infected_people;

    while total_infected <= num_reach {
        count_day += 1;
        let mut original_infected = total_infected;
        total_infected += infected_people * infections_per_person; 
        infected_people = total_infected - original_infected;
        //println!("Day:{}, Infected:{}, Total:{}", count_day, infected_people, total_infected)
    }
    println!("{count_day}");
}
