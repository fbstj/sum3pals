//! program for calculating the three palindromic numbers to sum into the number given as an argument

/// initial base
// TODO: allow customisation on args --base
const BASE: u8 = 10;

// storage type for each digit (must fit BASE)
type Digit = u8;

#[derive(Debug)]
struct Digits {
	digs: Vec<Digit>,
	base: Digit,
}

impl Digits {
	fn from(mut num: u128, base: Digit) -> Digits {
		let b : u128 = base as u128;
		let mut digits : Vec<u8> = vec![];

		while num >= b {
			digits.push((num % b) as u8);
			num = num / b;
		}

		digits.push(num as u8);
		digits.reverse();

		Digits { digs: digits, base }
	}
}

fn main() {
	let arg = std::env::args().nth(1)
				.expect("Please provide number to decompose into palindromes");
	let num : u128 = arg.parse()
				.expect("Please provide number to decompose into palindromes");
	let digits = Digits::from(num, BASE);
    println!("{:?}", digits);
}
