use std::io;

// P: Litros de pintura (1 <= P <= 100)
// B: Litros de pintura necesarios para convertir moneda (1 <= B <= 100)
// D: Pokedollars (1 <= D <= 100)
// Por cada litro de pintura que sobre > 1 D

// Input: (i32) P
//        (i32) B
//        (i32) D
// Output: i32 =  Amount of money James will make (in Pokedollars).  

fn main() {
    let mut numeros: [i32; 3] = [0 ; 3];

    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        numeros[i] = input.trim().parse().expect("Error al convertir en numero entero");
    }
    let cantidad_monedas = numeros[0] / numeros[1];
    let cantidad_pokedollars = cantidad_monedas * numeros[2];
    let cantidad_pintura_sobrante =  numeros[0] % numeros[1];
    let total = cantidad_pokedollars + cantidad_pintura_sobrante;
    
    //println!("{}", sobrante);
    println!("{}", total);

}
