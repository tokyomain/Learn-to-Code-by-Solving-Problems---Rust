// CCC '00 S1 - Slot Machines
// Problem Description: https://dmoj.ca/problem/ccc00s1

use std::io;

// Input A: int > quarters
// Input B: int > machine 1 to pay
// Input C: int > machine 2 to pay
// Input D: int > machine 3 to pay

fn main() {
    // Input A
    let mut quarters: String = String::new();
    io::stdin().read_line(&mut quarters).expect("Error al leer la entrada.");
    let mut quarters: i32 = quarters.trim().parse().expect("Error al convertir en entero.");
    
    // Input B
    let mut machine_1: String = String::new();
    io::stdin().read_line(&mut machine_1).expect("Error al leer la etnrada.");
    let mut machine_1: i32 = machine_1.trim().parse().expect("Error al convertir en entero.");
    
    // Input C
    let mut machine_2: String = String::new();
    io::stdin().read_line(&mut machine_2).expect("Error al leer la entrada.");
    let mut machine_2: i32 = machine_2.trim().parse().expect("Error al convertir en entero.");
    
    // Input D
    let mut machine_3: String = String::new();
    io::stdin().read_line(&mut machine_3).expect("Error al leer la entrada.");
    let mut machine_3: i32 = machine_3.trim().parse().expect("Error al convertirn en entero.");
    
    // Data > numbers of plays
    let mut plays: i32 = 0;

    // Data > machine being played
    let mut machine: i32 = 0;

    // Loop

    while quarters > 0 {
        quarters -= 1;

        if machine == 0 {
            machine_1 += 1;
            if machine_1 == 35 {
                machine_1 = 0;
                quarters += 30;
            }
        } else if machine == 1 {
            machine_2 += 1;
            if machine_2 == 100 {
                machine_2 = 0;
                quarters += 60;
            }
        } else if machine == 2 {
            machine_3 += 1;
            if machine_3 == 10 {
                machine_3 = 0;
                quarters += 9;
            }
        }
        plays += 1;
        machine += 1;
        if machine == 3 {
            machine = 0
        }
    }

    println!("Martha plays {} times before going broke.", plays);

}
