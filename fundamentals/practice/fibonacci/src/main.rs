//prompt: generate the nth fibonacci number
use std::io;

fn main() {
    
    let n  = input();

    let result = fibonacci(n);
    println!("{result}");
}

fn input() -> u64{
    //prompt user for n
    println!("Please input to what index of the Fibonacci sequence you'd like.");
    let mut n: String = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("'cmon mane..");
    
    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    n
}

fn fibonacci(n: u64) -> u128{
    let mut f1: u128 = 0;
    let mut f2: u128 = 1;
    let mut sum: u128 = 0;
    if n <= 0 {
        println!("Number must be greater than zero");
    } else if n == 1 {
        f2 = 0;
    } else if n == 2 {
        f2 = 1;
    } else {
        for index in (2..n).rev(){
            sum = f1 + f2;
            f1 = f2;
            f2 = sum;
        }
    }
    f2
}