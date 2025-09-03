fn main(){
    let x = 5;
    println!("The number is {x}");
    //cannot assign twice to immutable variable
    //x = 6;
    println!("The number is still {x}");

    //lets try making it mutable
    let mut y = 5;
    println!("The number is {y}");
    //since it was declared as mutable, we can change the value of y
    y = 6;
    println!("The new number is {y}");

    //you can use constants to define always immutable (annotated data type) values, capitalized and underscored naming convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");
    //constants are valid for the entire runtime within the scope it exists

    //shadowing a variable occurs when you reinstantiate with a different value, for instance:
    let n = 10;
    println!("n = {n}");
    let n = n + 1;
    println!("n = {n}");
    //the difference between making a variable mutable and shadowing, is that shadowing essentially makes a new variable wheras mutable alters the previous one
    //this can also be used on data types other than numbers
    let string = "test";
    let string = string.len();
    //the above changes string as a string to a integer called string
    //if we used mutable
    //ERROR EXAMPLE let mut test = "string";
    //ERROR EXAMPLE test = test.len();
    //the above gives a compiler error, as we cannot change the type of test from a string to an integer
}