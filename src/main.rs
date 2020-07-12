use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn select_difficulty() -> u32 {
    let mut difficulty = String::new();
    loop {
        println!("Please select a difficulty level :");
        println!("1- Easy : 1 to 50");
        println!("2- Medium : 1 to 100");
        println!("3- Hard : 1 to 200");
        println!("4- Insane : 1 to 500");

        io::stdin().read_line(&mut difficulty).expect("Failed to read line");

        let difficulty: u32 = match difficulty.trim().parse() {
            Ok(1) => 51,
            Ok(2) => 101,
            Ok(3) => 201,
            Ok(4) => 501,
            _ => {
                println!("Please enter a valid level !\n");
                continue;
            },
        };

        break rand::thread_rng().gen_range(1, difficulty);
    }
}

fn main() {
    println!("\tGuess the number !");
    let secret_number = select_difficulty();
    let mut count = 0;

    loop {
        println!("Please input your guess :");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number, please retry !\n");
                continue;
            },
        };
        
        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!\n");
                count = count + 1;
            },
            Ordering::Greater => {
                println!("Too big!\n");
                count = count + 1;
            },
            Ordering::Equal => {
                println!("\nYou win in {} hits !\n", count);
                break;
            }
        }
    }
}
