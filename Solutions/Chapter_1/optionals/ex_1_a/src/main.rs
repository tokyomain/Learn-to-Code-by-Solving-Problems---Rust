use std::io;

// S (2<= S <= 20)
// Input: int = 4
// Output: Spooooky  (4 -> o)

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let veces: usize = input.trim().parse().expect("Error al convertir en entero");

    let line_1 = String::from("sp");
    let line_2 = String::from("ky");

    let vector_o: String = vec!['o'; veces]
        .into_iter()
        .collect();

    let spooky_word = line_1 + &vector_o + &line_2;

    println!("{}", spooky_word)
}
