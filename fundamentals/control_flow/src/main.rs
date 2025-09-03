fn main() {
    let number = 3;

    if number != 3 {
        println!("The number is not 5!");
    } else if number % 3 == 0 {
        println!("The number divisible by 3!");
    } else {
        println!("ight idk");
    }

    if_in_a_let();
    loops();
    while_and_for();
}

//match is better than too many ifs
//you can use if in a let, types must match

fn if_in_a_let(){
    let condition = true;
    let number = if condition {5} else {6};

    println!("{number}");
}

//loops are lit

fn loops(){
    //loop will go until termination or break
    let mut counter = 0;
    let x = loop {
        counter += 1;
        
        if counter == 10 {
            //add desired return value after break statement
            break counter * 2;
        }
    };
    //use break to exit current loop, and return to exit current function

    println!("{x}");

    //you can label loops using the following syntax:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

//while and for loops

fn while_and_for(){
    let mut number = 3;

    while number != 0 {
        number -= 1;
        println!("{number}");
    }

    let a = [1, 2, 3, 4, 5];
    
    for element in a {
        println!("the current index is {element}");
    }

    //can use for loop for range as well (requires std lib range)
    for number in (1..4).rev() {
        //do stuff
    }
}

//machine code from for loops is often more efficient due to not needing to compare index to length of array at each iteration
