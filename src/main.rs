use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;

fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}
fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}
fn read_number(min: u64, max: u64) -> u64 {
    loop {
        println!("Enter a number from {} to {}:", min, max);
        let mut input = String::new();
        let bytes = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if bytes == 0 {
            std::process::exit(0);
        }
        match input.trim().parse::<u64>() {
            Ok(n) if n >= min && n <= max => return n,
            _ => println!("Invalid input. Try again."),
        }
    }
}
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for i in 0..count {
        total += roll_die(seed + i);
    }
    total
}
fn print_status(gold: u64, food: u64, day: u64) {
    println!("Gold: {} | Food: {} | Day: {}", gold, food, day);
}
fn print_menu() {
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}
fn main() {
    println!("Welcome to Trail Trader!");
    let seed = read_number(1, MAX_SEED);
    let mut gold: u64 = START_GOLD;
    let mut food: u64 = 0;
    let mut day: u64 = 1;
    let mut roll_count: u64 = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);
        if choice == 1 {
            if !can_afford(gold, FOOD_PRICE) {
                println!("You cannot afford any food.");
            } else {
                let max_units = gold / FOOD_PRICE;
                let units = read_number(1, max_units);
                gold -= units * FOOD_PRICE;
                food += units;
                day += 1;
            }
        } else if choice == 2 {
            let roll = roll_die(seed + roll_count);
            roll_count += 1;
            let gained = hunt_food(roll);
            food += gained;
            day += 1;
            println!("You rolled a {} and gained {} food.", roll, gained);
        } else if choice == 3 {
            let total = roll_many(seed + roll_count, 3);
            roll_count += 3;
            gold += 2 * total;
            day += 1;
            println!("The caravan paid you {} gold.", 2 * total);
        } else if choice == 4 {
            print_status(gold, food, day);
        } else {
            print_status(gold, food, day);
            break;
        }
    }
}
