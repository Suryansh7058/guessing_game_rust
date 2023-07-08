use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Enter the biggest number you want to guess: ");
    let mut biggest = String::new();
    io::stdin().read_line(&mut biggest).expect("Failed to read line");
    let biggest: u32 = biggest.trim().parse().expect("Please type a number");

    let secret_number = rand::thread_rng().gen_range(1..=biggest);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess :u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            },
        }
    }
}
