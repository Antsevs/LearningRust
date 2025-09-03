//generate the entire twelve days of christmas song
fn main() {
    let lyrics = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    for day in 0..days.len() {
        println!("On the {} day of Christmas, my true love gave to me:\n", days[day]);
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                println!("and {}\n", lyrics[gift]);
            } else {
                println!("{}", lyrics[gift])
            }
        }
    }
}
