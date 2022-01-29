use std::fmt::Display;

pub struct Number {
    pub base: u8,
    pub value: Vec<u8>,
}

impl Number {
    fn new(base: u8, value: Vec<u8>) -> Number {
        Number {
            base,
            value,
        }
    }

    fn to_base(&mut self, base: u8) {
        todo!()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Base : {}, Valeur : {:?}", self.base, convert::to_string(self))
    }
}

pub mod convert {
    use std::vec;

    use crate::Number;

    pub static SYMBOLS: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/".as_bytes();

    pub fn fix_base(base: u8) -> u8 {
        let test_base = 2;
        while test_base <= 64 {
            if test_base == base {
                return base;
            }
        }
        16
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

    pub fn to_u64(nbr: &Number) -> u64 {
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
}
