use base_converter::{Number, convert};

fn main() {
    let number = convert::to_number(12, 2);

    println!("{}", convert::to_u64(&number));

    let number = convert::from_string(String::from("111111"), 2);

    println!("{}", number);

    println!("{}", convert::to_u64(&number));
}