// CCC '15 J2 - Happy or Sad
// Problem description: https://dmoj.ca/problem/ccc15j2

use std::io;

fn determine_mood(message: &str) -> &'static str {
    let happy_count = message.matches(":-)").count();
    let sad_count = message.matches(":-(").count();

    if happy_count == 0 && sad_count == 0 {
        "none"
    } else if happy_count == sad_count {
        "unsure"
    } else if happy_count > sad_count {
        "happy"
    } else {
        "sad"
    }
}

fn main() {
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Error al leer la entrada");

    let mood = determine_mood(&message);
    println!("{}", mood);

}
