use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let seret_num: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guss = String::new();
        io::stdin()
            .read_line(&mut guss)
            .expect("Faild to read line.");

        let guss: u32 = match guss.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please pass valid number, ");
                continue;
            }
        };

        match guss.cmp(&seret_num) {
            Ordering::Less => println!("{}", "Too small!!".yellow()),
            Ordering::Greater => println!("{}", "Too Large!!!".red()),
            Ordering::Equal => {
                println!("{}", "You are Win...".green());
                break;
            }
        }
    }
}
