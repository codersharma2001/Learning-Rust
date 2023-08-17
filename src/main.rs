//guessing game
use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;
fn main(){
    println!("Welcome to guessing game");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Secret Number is : {} " , secret_number);
   
    loop{
         
    println!("Please input your guess");
    let mut guess = String ::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {}",guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // converting to integer
    // println!("{}",guess+1);
    match guess.cmp(&secret_number){
        std::cmp::Ordering::Less => println!("Too small"),
        std::cmp::Ordering::Greater => println!("Too big"),
        std::cmp::Ordering::Equal => {
            println!("You win");
            break;
        },
    }
    }
}