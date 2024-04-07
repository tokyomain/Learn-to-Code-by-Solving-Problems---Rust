use std::io;

fn main() {
    // Leer la contraseña desde la entrada estándar
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let password = input.trim();

    // Verificar la longitud de la contraseña
    match password.len() {
        8..=12 => (), // Longitud válida, no hacemos nada
        _ => {
            println!("Invalid");
            return;
        }
    }

    // Contar las ocurrencias de letras mayúsculas, minúsculas y dígitos
    let mut uppercase_count = 0;
    let mut lowercase_count = 0;
    let mut digit_count = 0;

    for c in password.chars() {
        match c {
            'A'..='Z' => uppercase_count += 1,
            'a'..='z' => lowercase_count += 1,
            '0'..='9' => digit_count += 1,
            _ => {
                println!("Invalid");
                return;
            }
        }
    }

    // Verificar si cumple con los requisitos de la contraseña
    if uppercase_count >= 2 && lowercase_count >= 3 && digit_count >= 1 {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}