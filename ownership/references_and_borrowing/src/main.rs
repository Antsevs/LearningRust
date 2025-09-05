//references can be used to avoid granting owenership of data to function (pasically a ptr to a ptr)
fn main() {
    let s1 = String::from("hello");

    let len = calculate_len(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

//you cannot modify borrowed (referenced) data. error example below:
fn og() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    //some_string.push_str(", world");
}

//you can fix the above by using a mutable reference
//the restriction with mutable references is that the data it references cannot have any other references (mut or not) (prevents data races)
fn og2() {
    let mut s = String::from("hello");
    change2(&mut s);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

//Rust does not allow for dangling references