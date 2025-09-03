//these are import statements
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number! ");
    //thread_rng is the number generator, gen_range is the range to generate
    //thread_rng is local to current thread of execution and seeded by OS
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();  //defines a mutable var, of size empty string
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            //read_line grabs input and ties to new string
        println!("You guessed: {guess}");

        //guess was mutable, now use let to "shadow" guess into new value
        //.trim() to remove whitespace, .parse() to convert string into some other type
        //converting string to u32 int below!!
        let guess: u32 = match guess.trim().parse(){        
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big.")
        }
    }
}