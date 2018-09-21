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

/// classification of digits
#[derive(Debug)]
enum Class {
	/// numbers with <7 digits
	Small,
	/// numbers of type A.1
	/// - second most significant digit is >2
	/// - D(least-sig  - most-sig - 2nd-most-sig + 1) != 0
	TypeA1,
	/// numbers of type A.2
	/// - second most significant digit is >2
	/// - D(least-sig  - most-sig - 2nd-most-sig digit + 1) == 0
	TypeA2,
	/// numbers of type A.3
	/// - second most significant digit is {0,1,2}
	/// - most significant digit not 1
	/// - D(least-sig - most-sig + 2) != 0
	TypeA3,
	/// numbers of type A.4
	/// - second most significant digit is {0,1,2}
	/// - most significant digit not 1
	/// - D(least-sig - most-sig + 2) == 0
	TypeA4,
	/// numbers of type A.5
	/// - most-sig digits are 10
	/// - 3rd-most-sig {0,1,2,3}
	/// - D(least-sig - 3rd-most-sig) != 0
	TypeA5,
	/// numbers of type A.6
	/// - most-sig digits are 10
	/// - 3rd-most-sig {0,1,2}
	/// - D(least-sig - 3rd-most-sig) == 0
	TypeA6,
	/// numbers of type B.1
	/// - most-sig digit is 1
	/// - 2nd-most-sig {0,1,2}
	/// - 3rd-most-sig not {0,1,2,3}
	/// - D(least-sig - 3rd-most-sig) != 0
	TypeB1,
	/// numbers of type B.2
	/// - most-sig digit is 1
	/// - 2nd-most-sig {0,1,2}
	/// - 3rd-most-sig not {0,1,2}
	/// - D(least-sig - 3rd-most-sig) == 0
	TypeB2,
	/// numbers of type B.3
	/// - most-sig digit is 1
	/// - 2nd-most-sig digit is {1,2}
	/// - 3rd-most-sig digit is {0,1}
	/// - lesat-sig is 0
	TypeB3,
	/// numbers of type B.4
	/// - most-sig digit is 1
	/// - 2nd-most-sig digit is {1,2}
	/// - 3rd-most-sig digit is {2,3}
	/// - lesat-sig is 0
	TypeB4,
	/// numbers of type B.5
	/// - most-sig digit is 1
	/// - 2nd-most-sig digit is {1,2}
	/// - 3rd-most-sig digit is {0,1,2}
	/// - least-sig not 0
	TypeB5,
	/// numbers of type B.6
	/// - most-sig digit is 1
	/// - 2nd-most-sig digit is {1,2}
	/// - 3rd-most-sig digit is 3
	/// - D(least-sig - 3) not 0
	TypeB6,
	/// numbers of type B.7
	/// - most-sig digit is 1
	/// - 2nd-most-sig digit is {1,2}
	/// - 3rd-most-sig digit is 3
	/// - lesat-sig is 3
	TypeB7,
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

	fn classify(&self) -> Class {
		if self.digs.len() < 7 {
			return Class::Small;
		}
		unimplemented!();
	}
}

fn main() {
	let arg = std::env::args().nth(1)
				.expect("Please provide number to decompose into palindromes");
	let num : u128 = arg.parse()
				.expect("Please provide number to decompose into palindromes");
	let digits = Digits::from(num, BASE);
    println!("{:?}", digits);
    println!("{:?}", digits.classify());
}
