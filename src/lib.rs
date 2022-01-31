use std::{env, error::Error, fmt::Display};

#[derive(Debug)]
pub struct Number {
    pub base: u8,
    pub value: Vec<u8>,
}

impl Number {
    fn new(base: u8, value: Vec<u8>) -> Number {
        let value = convert::remove_zeros(value);
        Number { base, value }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", convert::to_string(self), self.base,)
    }
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Convert,
}

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub base: Option<u8>,
    pub number1: Number,
    pub number2: Option<Number>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let operation = match args.next() {
            Some(op) => match op.as_str() {
                "add" => Operation::Add,
                "convert" => Operation::Convert,
                _ => return Err("Unknown command"),
            },
            None => return Err("No command specified"),
        };

        let number1 = match (args.next(), args.next()) {
            (Some(x), Some(base)) if base == "10" => {
                convert::to_number(u64::from_str_radix(&x, 10).unwrap(), 16)
            },
            (Some(x), Some(base)) => {
                let base = u8::from_str_radix(&base, 10);

                match base {
                    Ok(base) if convert::is_base_valid(base) => convert::from_string(x, base).unwrap(),
                    _ => return Err("First number parse error")
                }
            }
            _ => return Err("Not enough arguments"),
        };

        let mut number2 = None;
        let mut base = None;
        match operation {
            Operation::Add => {
                number2 = match (args.next(), args.next()) {
                    (Some(x), Some(base)) if base == "10" => {
                        Some(convert::to_number(u64::from_str_radix(&x, 10).unwrap(), 16))
                    },
                    (Some(x), Some(base)) => {
                        let base = u8::from_str_radix(&base, 10);
        
                        match base {
                            Ok(base) if convert::is_base_valid(base) => Some(convert::from_string(x, base).unwrap()),
                            _ => return Err("Second number parse error")
                        }
                    }
                    _ => return Err("Not enough arguments"),
                };
            }
            Operation::Convert => {
                base = match args.next() {
                    Some(base) => {
                        let base = u8::from_str_radix(&base, 10);
        
                        match base {
                            Ok(base) if convert::is_base_valid(base) => Some(base),
                            _ => return Err("Destination base parsing error")
                        }
                    },
                    _ => return Err("Destination base was not provided"),
                };
            }
        }

        Ok(Config {
            operation,
            number1,
            number2,
            base,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result = process_command(&config);

    match config.operation {
        Operation::Add => println!(
            "{}",
            result
        ),
        Operation::Convert => println!("{}", result),
    };

    Ok(())
}

pub fn process_command(config: &Config) -> Number {
    match config.operation {
        Operation::Add => convert::add_number(&config.number1, config.number2.as_ref().unwrap()),
        Operation::Convert => convert::to_base(&config.number1, config.base.unwrap()),
    }
}

pub mod convert {
    use std::vec;

    use crate::Number;

    pub static SYMBOLS: &[u8] =
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/".as_bytes();

    pub fn is_base_valid(base: u8) -> bool {
        let mut test_base = 2;
        while test_base <= 64 {
            if test_base == base {
                return true;
            } else {
                test_base *= 2;
            }
        }

        false
    }

    pub fn fix_base(base: u8) -> u8 {
        let mut test_base = 2;
        while test_base <= 64 {
            if test_base == base {
                return base;
            } else {
                test_base *= 2;
            }
        }
        16
    }

    pub fn remove_zeros(value: Vec<u8>) -> Vec<u8> {
        let mut v = value[..].to_vec();

        if value.len() > 1 {
            for &i in value.iter().rev() {
                if i != 0 || v.len() == 1 {
                    break;
                } else {
                    v.pop();
                }
            }
        }

        v
    }

    pub fn to_number(nbr: u64, base: u8) -> Number {
        let base: u8 = fix_base(base).into();

        let mut bigger_power: u64 = base.into();
        let mut max_index: u64 = 1;

        while nbr > bigger_power {
            bigger_power = bigger_power * (base as u64);
            max_index += 1;
        }

        let mut quotient: u64 = nbr;
        let mut remainder;
        let mut value: Vec<u8> = vec![];
        for _ in 0..max_index {
            remainder = (quotient % base as u64) as u8;
            quotient /= base as u64;
            value.push(remainder.into());
        }

        Number::new(base, value)
    }

    pub fn to_uint(nbr: &Number) -> u64 {
        let mut value = 0u64;

        for i in 0..nbr.value.len() as u32 {
            value += (nbr.value[i as usize] as u64) * (nbr.base as u64).pow(i);
        }

        value
    }

    pub fn to_string(nbr: &Number) -> String {
        let value_length = nbr.value.len();

        let mut string_rep = vec![0u8; value_length];

        for i in (0..value_length).rev() {
            string_rep[value_length - i - 1] = SYMBOLS[nbr.value[i] as usize];
        }

        String::from_utf8(string_rep).unwrap()
    }

    pub fn from_string(value: String, base: u8) -> Option<Number> {
        let value = value.as_bytes();

        let mut representation = vec![0u8; value.len()];

        let base = fix_base(base);

        for i in 0..value.len() {
            let mut val = 0u8;

            for (index, &c) in SYMBOLS.iter().enumerate() {
                if c == value[i] {
                    val = index as u8;
                    break;
                }
            }

            if val > base {
                return None;
            }

            representation[value.len() - i - 1] = val;
        }

        Some(Number::new(base, representation))
    }

    pub fn to_base(nbr: &Number, base: u8) -> Number {
        let base = fix_base(base);

        if base == nbr.base {
            return Number::new(base, nbr.value[..].to_vec());
        }

        let mut binary_rep: Vec<u8>;

        if nbr.base != 2 {
            let ratio = (nbr.base as f32).log2() as u8;
            binary_rep = vec![0u8; ratio as usize * nbr.value.len()];

            for (i, &val) in nbr.value.iter().enumerate() {
                let mut remainder = val;

                for j in 0..ratio {
                    binary_rep[i * (ratio as usize) + j as usize] = remainder % 2u8;
                    remainder /= 2u8;
                }
            }
        } else {
            binary_rep = nbr.value[..].to_vec();
        }

        let mut value: Vec<u8>;

        if base != 2 {
            let ratio = (base as f32).log2() as u8;
            let length = binary_rep.len() / ratio as usize + 1;
            value = vec![0u8; length];

            for i in 0..length {
                let mut val = 0u8;

                for j in 0..(ratio as u32) {
                    val += match binary_rep.get(i * ratio as usize + j as usize) {
                        Some(&val) => val * 2u8.pow(j),
                        None => 0,
                    };
                }

                value[i] = val;
            }
        } else {
            value = binary_rep;
        }

        Number::new(base, value)
    }

    pub fn add_number(a: &Number, b: &Number) -> Number {
        let base = a.base;
        let b = to_base(&b, base);

        let length = a.value.len().max(b.value.len());
        let mut result = vec![0u8; length + 2usize];
        let mut remainder = 0u8;
        for i in 0..=length {
            let val = match (a.value.get(i), b.value.get(i)) {
                (None, None) => remainder,
                (Some(&x), None) => x + remainder,
                (None, Some(&y)) => y + remainder,
                (Some(&x), Some(&y)) => x + y + remainder,
            };

            remainder = val / base;

            result[i] = val % base;
        }

        Number::new(base, result)
    }

    /// Returns -1 if a < b, 0 if a == b, +1 if a > b
    pub fn cmp_number(a: &Number, b: &Number) -> i8 {
        let base = a.base;
        let b = to_base(b, base);
        println!("{} {}", a, b);

        if a.value.len() > b.value.len() {
            return 1;
        } else if a.value.len() < b.value.len() {
            return -1;
        }

        let length = a.value.len();
        for i in (0..length).rev() {
            if a.value[i] > b.value[i] {
                return 1;
            } else if a.value[i] < b.value[i] {
                return -1;
            }
        }

        0
    }
}
