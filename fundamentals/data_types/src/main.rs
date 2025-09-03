fn main(){
    //rust is statically typed (must know type of all vars at compile time)
    //the compiler can make this decision when its unsure, but we can define type by annotating
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    //Types include:
        //Scalar types
            //integer(i8-i128 | u8-u128 | isize-usize (architecture dependent))
                //signed values stored using two's complement
                //integer literals (can use _ to separate like a comma would):
                    //Decimal (98_222) = 98,222 | (2526) = 2,526
                    //Hex (0xff)
                    //Octal (0o77)
                    //Binary (0b1111_0000) = 11110000
                    //Byte, limited to u8 (b'A')        
                    //At compile time, integer overflow caught as error, but in release, it doesn't and simply overflows.
                    //libraries to handle the above are wrapping_*, checked_*, overflowing_*, and saturating_* (each have )  
                //float are always signed f32 or f64 (same speed, 64 more precision)
    let x = 2.0;
    let y: f32 = 3.0;   //appended to make 32 bit

    println!("{}", -5.0 / 3.0);
                //booleans are 1 byte
                //chars use '' instead of "" for string literals, and is four bytes
        //Compound types
            //Tuple (grouping a number of values with variety of types into one compound type)
                //Once declared cannot shrink or grow in size
                //define first with type index (annotating), then values
    let tup: (i32, f64, u8) = (500, 6.4, 1);
                //to grab values of a tuple, pattern match (called destructuring)
    let(x, y, z) = tup;
    println!("{}{}{}", x, y, z);
                //or use dot index
    let index_one = tup.0;
    let index_two = tup.1;
    let index_three = tup.2;
    println!("The tuple indexes are {index_one}, {index_two}, and {index_three}.");
                //empty tuples are called a unit
            //Array (grouping of a number of values with a single type)
                //fixed length, stored on stack
    let a = [1, 2, 3, 4, 5];
            //Vector (non native, require standard library)
                //dynamically sized, stored on heap
                //can annotate, but include size of array as well
    let b: [i32; 5] = [1, 2, 3, 4, 5];
                //can also fill an array with values
    let filled = [3; 5];
                //to access array elements
    let first = filled[0];
    let second = filled[1];
    println!("First is {first} and second is {second}.")
}