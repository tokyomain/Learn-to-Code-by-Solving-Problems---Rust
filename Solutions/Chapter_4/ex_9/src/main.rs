// CCC '08 J2 - Do the Shuffle
// Problem Description: https://dmoj.ca/problem/ccc08j2
use std::io;


fn main() {
    let mut playlist = vec!["A", "B", "C", "D", "E"];

    loop {

        let mut button = String::new();
        io::stdin().read_line(&mut button).expect("Error al leer la entrada.");
        let button = button.trim().parse().expect("Error al convertir en entero.");
    
        let mut number_press = String::new();
        io::stdin().read_line(&mut number_press).expect("Error al leer la entrada.");
        let number_press = number_press.trim().parse().expect("Error al convertir en entero.");

        match button {
            // Button 1: move the first song to the end
            1 => { for _times in 0..number_press{
                 playlist.rotate_left(1)}
                },
            // Button 2: move the last song to the start 
            2 => {for _times in 0..number_press {
               playlist.rotate_right(1)}
                },
            // Button 3: swap the first two songs 
            3 => {for _times in 0..number_press {
                playlist.swap(0, 1)}
            }
            // Button 4: break
            4 => {break},
            _ => {},
        }
    }
    for song in playlist {
        print!("{} ", song)
    }
    
}
