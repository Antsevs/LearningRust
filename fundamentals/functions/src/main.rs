fn main(){
    println!("Hello world!");

    another_function(5, 'h');

    let x = five();
    println!("{x}")

    let n = mismatched_types(5);
    println!("{n}");    //this will throw an error of mismatched types. 
}

//parameters MUST be annotated
fn another_function(payment: i32, item: char){
    println!("You paid {payment} dollars, and bought a {item}.");
}

//statements are instructions that perform an action and return no value
//expressions evaluate a resultant value, they do not have a semicolon at the end

fn statement_vs_expression(){   //statement
    let y = 5;  //this is a statement
    let z = {   //------------------------ expression
        let x = 5;  //statement         --
        x + 1   //expression            --
    };          //------------------------ expression
    println!("{}", z)   //expression
}

//functions will return last value if instantiated with -> arrow operator (declares type)

fn five() -> i32{
    5
}

fn mismatched_types(n: i32) -> i32{
    n + 1;  //statement doesn't return value
}