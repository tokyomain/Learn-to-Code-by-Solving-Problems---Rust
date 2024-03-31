use std::io;

// Input: 2 Numeros enteros en lineas diferentes
// 1st:  Y > age of youngest child (0 <= Y <= 50)
// 2nd:  M > age of middle child (Y <= M <= 50)
// Output: The output will be the age of the oldest child.

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let age_y: i32 = input.trim().parse().expect("Error al convertir a entero");
    input.clear();

    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let age_m: i32 = input.trim().parse().expect("Error al convertir en entero");

    let age_o: i32 = (age_m - age_y) + age_m;
    println!("{}", age_o);
}
