use rand::Rng; 
use std::io::{stdin}; 

fn main() {
    println!("Welcome to the Guess the Number!");
    let mut rng = rand::thread_rng();
    let target: i32 = rng.gen_range(1..10); 
   
    loop { 
        println!("Type the number : ");
        let mut input = String::new();  
        stdin().read_line(&mut input).unwrap(); 

        let input: i32 = match input.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter the valid number!!");
                return; 
            }
        };

        if target == input {
            println!("You are Correct!");
            println!("The number was : {}",target); 
            break; 
        }
        if target > input { 
            println!("Up!");
        }else{
            println!("Down!");
        }

    }
   
}
