use std::io;

// Escribe un programa que compute las calorias totales de una comida
// Input: int > 4 lineas > Un numero para cada eleccion.
//    1> Burger 2> Side 3> Drink 4> Dessert
// Output: Your total Calorie count is 649. > Calorias totales

struct Burger {
    cheesegurger: i32,
    fish_burger: i32,
    veggie_burger: i32,
    no_burger: i32,
}

impl Burger {
    const CHEESEBURGER: i32 = 461;
    const FISH_BURGER: i32 = 431;
    const VEGGIE_BURGER: i32 = 420;
    const NO_BURGER: i32 = 0;
}

struct Side {
    fries: i32,
    baked_potato: i32,
    chef_salad: i32,
    no_side_order: i32,
}

impl Side {
    const FRIES: i32 = 100;
    const BAKED_POTATO: i32 = 57;
    const CHEF_SALADA: i32 = 70;
    const NO_SIDE_ORDER: i32 = 0;
}

struct Drink {
    soft_drink: i32,
    orange_juice: i32,
    milk: i32,
    no_drink: i32,
}

impl Drink {
    const SOFT_DRINK: i32 = 130;
    const ORANGE_JUICE: i32 = 160;
    const MILK: i32 = 118;
    const NO_DRINK: i32 = 0;
}

struct Dessert {
    apple_pie: i32,
    sundae: i32,
    fruit_cup: i32,
    no_dessert: i32,
}

impl Dessert {
    const APPLE_PIE: i32 = 167;
    const SUNDAE: i32 = 266;
    const FRUIT_CUP: i32 = 75;
    const NO_DESSERT: i32 = 0;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let burger_choice: usize = input.trim().parse().expect("Input invalido");

    input.clear();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let side_choice: usize = input.trim().parse().expect("Input invalido");

    input.clear();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let drink_choice: usize = input.trim().parse().expect("Input invalido");

    input.clear();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let dessert_choice: usize = input.trim().parse().expect("Input invalido");

    let burger_cost = match burger_choice {
        1 => Burger::CHEESEBURGER,
        2 => Burger::FISH_BURGER,
        3 => Burger::VEGGIE_BURGER,
        _ => Burger::NO_BURGER,
    };

    let side_cost = match side_choice {
        1 => Side::FRIES,
        2 => Side::BAKED_POTATO,
        3 => Side::CHEF_SALADA,
        _ => Side::NO_SIDE_ORDER
    };

    let drink_cost = match drink_choice {
        1 => Drink::SOFT_DRINK,
        2 => Drink::ORANGE_JUICE,
        3 => Drink::MILK,
        _ => Drink::NO_DRINK,
    };

    let dessert_cost = match dessert_choice {
        1 => Dessert::APPLE_PIE,
        2 => Dessert:: SUNDAE,
        3 => Dessert:: FRUIT_CUP,
        _ => Dessert::NO_DESSERT,
    };

    let total_cost = burger_cost + side_cost + drink_cost + dessert_cost;

    println!("Your total Calorie count is {total_cost}.");
}
