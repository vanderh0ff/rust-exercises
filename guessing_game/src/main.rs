use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Put forth your prediction!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Please input your number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =  match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Predict for a greater future!"),
            Ordering::Greater => println!("You Presume much!"),
            Ordering::Equal => {
                println!("You are correct!");
                break;
            }
        }
    }
}
