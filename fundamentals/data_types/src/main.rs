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
}