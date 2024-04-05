// COCI '16 Contest 1 #1 Tarifa
// Problem Description: https://dmoj.ca/problem/coci16c1p1
// Input:   1- int: X(1<= X <= 100) > Mb available per month
//          2- int: N(1<= N <= 100) > Number of months
//          N- int: Pi(0<= Pi <= 10.000) > Number of mb spent in each month
// Output: int: Number of mb available to spend in the N+1 month.

use std::io::stdin;

fn main() {
    let mut mb_disponibles = String::new();
    stdin().read_line(&mut mb_disponibles).expect("Error al leer la entrada");
    let mb_disponibles: i32 = mb_disponibles.trim().parse().expect("Error al convertir en entero");

    let mut num_meses = String::new();
    stdin().read_line(&mut num_meses).expect("Error al leer la entrada");
    let num_meses = num_meses.trim().parse().expect("Error al convertir en entero");

    let mut mb_usados_mes: Vec<i32> = Vec::new();

    for _ in 0..num_meses {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error al leer la entrada");
        let input: i32 = input.trim().parse().expect("Error al convertir en entero");
        mb_usados_mes.push(input);
    }

    let mut mb_acumulados = 0;

    for valor in mb_usados_mes {
        mb_acumulados = (mb_disponibles + mb_acumulados) - valor;
    }
    println!("{}", mb_acumulados + mb_disponibles);
}
