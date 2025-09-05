//ownership is the alternative for managing memory to a garbage collector or alloc
//memory still stored in stack (LFIO) and heap (pushing vals returning pointer)

//OWERNSHIP RULES
    //Every value has owner
    //there can only be one owner at a time
    //when owner goes out of scope, value is dropped

//String data type can manate data allocated on heap and store dynamic text sizes 
fn main() {
    let s = "hello";    //string literal, stored on stack
    let x = 5;
    let mut n = String::from("hello");  //String type, can be mutable, stored on heap using memory allocator
    n.push_str(", world!"); //this is essentially concatenating
    //instead of deallocating used memory, it is automatically returned once the variable goes out of scope (rust calls drop)

    takes_ownership(n); //n already came into scope, now its value moves into function, cannot use after

    makes_copy(x);  //x is stack data, copies into function, can be used after
    //this behavior functions the same with returns

    //rather than giving and returning heap data each time (not give ownership to function of data), can use tuple (complex), or references
    //references are explained next
}

fn move_example() {
    //create x , copy x to y
    let x = 5;
    let y = x;
    //create s1, copy s1 to s2
    let s1 = String::from("hello");
    let s2 = s1;
    //the above doesn't quite do that. String stores: ptr, length (how much memory contents of String uses), and capacity (total memory 
    //received from allocator by String)
    //so instead, we are copying the reference data of String to s2, rather than the distinct value itself like in the integer example
    //i.e., an additional piece of data is not created on the heap, rather, it references the same data as previous
    //after s1 = s2, rust considers s1 invalid, println!(s1); after this would throw an error.
    //Thus, better phrasing would be "We are moving s1 to s2", rather than "We are copying s1 to s2".
    //if we do want to deep copy heap data, we can use clone
    let s3 = String::from("Whats up");
    let s4 = s3.clone();    //now, both s3 and s4 contents are located on the heap
    //can use copy on stack data to trivially copy, rather than move variables
}

//takes heap data as parameter
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

//takes stack data as parameter
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
