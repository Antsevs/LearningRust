//prompt: Convert temperatures between celsius and fahrenheit
use std::io;

fn main(){

    println!("Enter 1 if you are inputting F, and 2 if you are inputting C.");

    let mut user_val: String = String::new();
    io::stdin()
        .read_line(&mut user_val)
        .expect("Please enter either 1 or 2 only.");
    let user_val: u8 = match user_val.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    let temp_in: f32 = input();

    if user_val == 1 {
        let f = f_to_c(temp_in);
        println!("{temp_in} degrees Fahrenheit is equivalent to {f} degrees Celsius.");
    } else if user_val == 2 {
        let c = c_to_f(temp_in);
        println!("{temp_in} degrees Celsius is equivalent to {c} degrees Fahrenhneit.");
    } else {
        println!("Something went wrong");
    }

}

fn input() -> f32{
    //take input and convert to f32 
    println!("Please enter a temperature");
    let mut temp: String = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = match temp.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0,
    };
    temp
}

fn f_to_c(temp: f32) -> f32{
    (temp - 32.0) / 1.8
}

fn c_to_f(temp: f32) -> f32{
    (temp * 1.8) + 32.0
}