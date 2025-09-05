//slice references contiguous sequence of elements in collection

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear();  //empties string, equivalent to ""
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();   //separates contents of string into array of bytes

    for (i, &item) in bytes.iter().enumerate() {    //for (index and tuple byte) in bytes array(return each element)(wrap each elemtn as part of tuple)
        //the above essentially calculates index (iter and enumerate)
        if item == b' ' {
            return i;
        }
    }

    s.len()
}