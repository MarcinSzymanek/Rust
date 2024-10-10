use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 and 100");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    fn check_results(guess_number: i32, target: i32) -> bool{

        match guess_number.cmp(&target){
            Ordering::Equal => {
                println!("JACKPOT!");
                return true;
            },
            Ordering::Greater => {
                println!("Your guess was too big.");
                return false;
            },
            Ordering::Less => {
                println!("Your guess was too small.");
                return false;
            }
        }
    }

    let max_guesses = 10;
    let mut guesses_left: i32 = max_guesses;
    let mut won: bool = false;

    loop{
        let mut guess: String = String::new();
        io::stdin()
            .read_line(& mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<i32>(){
            Ok(guessed_number) => {
                if check_results(guessed_number, secret_number){
                    won = true;
                    break;
                }
                guesses_left -= 1;

                if guesses_left == 0{
                    break;
                }
            }
            Err(_error) => {

                println!("Please enter a number!");
                continue;
            }
        }


    }
    match won{
        true => println!("You guessed correctly! WINNER IS YOU!"),
        false => println!("You lose :((")
    }




}
