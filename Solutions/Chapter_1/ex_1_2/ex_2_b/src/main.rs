use std::{f64::consts::PI, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let radius: f64 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Error parsing radius"),
        _ => panic!("Error reading radius"),
    };

    let height: f64 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Error parsing height"),
        _ => panic!("Error reading height"),
    };

    let volume = PI * radius.powi(2) * height / 3.0;

    println!("{:.2}", volume); // .2 > precisión de dos decimales
}
