use std::io::{self, BufRead};


struct NumeroTelefono {
    n_1:i32,
    n_2:i32,
    n_3:i32,
    n_4:i32,
}


fn main() {

    let mut numeros = Vec::new();

    for line in io::stdin().lock().lines().take(4) {
        let numero: i32 = line.expect("Error al leer la entrada")
            .trim().parse().expect("Error al convertir en entero.");
        numeros.push(numero)
    }

    let telefono = NumeroTelefono {
        n_1: numeros[0],
        n_2: numeros[1],
        n_3: numeros[2],
        n_4: numeros[3],
    };

    match telefono {
        NumeroTelefono { n_1: 8 | 9, n_4: 8 | 9, ..} //  El .. indica que no estás especificando explícitamente todos los campos de la estructura.
            if telefono.n_2 == telefono.n_3 => {
                println!("ignore");
            }        
            _ => {
                println!("answer");
        }
    }
    
}
