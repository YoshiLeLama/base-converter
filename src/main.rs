use base_converter::{convert, Number};

fn main() {
    let number = convert::to_number(12, 2);

    println!("{}", convert::to_u64(&number));

    let number = convert::from_string(String::from("10000000000000"), 2);

    if let Some(number) = number {
        println!("- {}", number);

        println!("-- {}", convert::to_u64(&number));
    }
}