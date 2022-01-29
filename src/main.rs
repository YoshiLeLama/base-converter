use base_converter::convert;

fn main() {
    let number = convert::to_number(165165163, 16);
    let number1 = convert::to_number(531311, 8);

    println!("{}", convert::to_uint(&convert::add_number(&number, number1)));
    
    let number = convert::to_base(&number, 8);

    println!("{}", number);
}