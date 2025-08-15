use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    let number: u16 = rand::random_range(0..100);
    loop {
        print!("Enter guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Too Low!");
            }
            Ordering::Greater => {
                println!("Too high!");
            }
            Ordering::Equal => {
                println!("Yay you win! The number was {number}");
                break;
            }
        }
    }
}
