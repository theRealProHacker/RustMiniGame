/*
A small mini-game
*/
// Can't use std in OS
// Also we cannot just panic
use std::{io, thread, time};
use rand::{thread_rng, Rng};

fn main() {
    const CHANCES: u8 = 7;
    let stdin = io::stdin();
    let rand_num: u8 = thread_rng().gen_range(1..=100);
    println!("Hello, do you want to play a game?");
    thread::sleep(time::Duration::from_secs(1));
    println!("I don't really care anyway. Guess the number between 1 and 100! You have {CHANCES} chances");
    thread::sleep(time::Duration::from_secs(1));
    for guess_count in 1..=CHANCES {
        println!("Guess {guess_count}:");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Couldn't read your input. Terminating"); // or use unwrap
        // trimming the input is really important, 
        // because the console input also contains invisible whitespace, 
        // which is not handled by parse
        match input.trim().parse::<u8>() {
            Ok(number)=>{
                if number == rand_num { 
                    println!("Correct!");
                    break;
                } else { 
                    let comp = if rand_num<number {"smaller"} else {"larger"};
                    println!("Wrong! The number is {comp}");
                };
            }
            Err(err) => {
                println!("Invalid input {:?}", err.kind());
            }
        }
    }
    println!("The random number is {rand_num}")
}
