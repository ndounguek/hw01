use std::io;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
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
fn main() {
   fn print_menu() {
       println!("1. Buy food");
       println!("2. Hunt");
       println!("3. Trade with caravan");
       println!("4. View status");
       println!("5. Quit");
   }
}
