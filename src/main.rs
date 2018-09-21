//! program for calculating the three palindromic numbers to sum into the number given as an argument

/// initial base
// TODO: allow customisation on args --base
const BASE: u8 = 10;

// storage type for each digit (must fit BASE)
type Digit = u8;

/// classification of numbers from Section 2.1 & 2.2
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Debug)]
struct Number {
	digs: Vec<Digit>,
	base: Digit,
}

impl Number {
	fn from(mut num: u128, base: Digit) -> Number {
		let b : u128 = base as u128;
		let mut digits : Vec<u8> = vec![];

		while num >= b {
			digits.push((num % b) as u8);
			num = num / b;
		}

		digits.push(num as u8);
		digits.reverse();

		Number { digs: digits, base }
	}

	/// fetch the nth most significant digit
	fn most_sig(&self, nth: usize) -> Digit {
		assert!(self.digs.len() > nth, "out of range");
		self.digs[self.digs.len() - nth]
	}
	/// fetch the nth least significant digit
	fn least_sig(&self, nth: usize) -> Digit {
		assert!(self.digs.len() > nth, "out of range");
		self.digs[nth - 1]
	}

	fn as_u128(&self) -> u128 {
		let g = self.base as u128;
		self.digs
			.iter()
			.enumerate()
			.map(|(n, dig)| *dig as u128 * g.pow(n as u32) )
			.sum()
	}

	fn classify(&self) -> Class {
		if self.digs.len() < 7 {
			return Class::Small;
		}
		// most significant 3 digits
		let m1 = self.most_sig(1);
		let m2 = self.most_sig(2);
		let m3 = self.most_sig(3);
		// least significant digit
		let l1 = self.least_sig(1);
		// D() calculations
		let g = self.base as isize;
		let d_l1_m1_m2_1 = ((l1 as isize) - (m1 as isize) - (m2 as isize) + 1) % g;
		let d_l1_m1_2 = ((l1 as isize) - (m1 as isize) + 2) % g;
		let d_l1_m3 = ((l1 as isize) - (m3 as isize)) % g;
		let d_l1_3 = ((l1 as isize)- 3) % g;
		// type A numbers
		if !(m2 == 0 || m2 == 1 || m2 == 2) && d_l1_m1_m2_1 != 0 {
			Class::TypeA1
		} else
		if !(m2 == 0 || m2 == 1 || m2 == 2) && d_l1_m1_m2_1 == 0 {
			Class::TypeA2
		} else
		if (m2 == 0 || m2 == 1 || m2 == 2) && m1 != 1 && d_l1_m1_2 != 0 {
			Class::TypeA3
		} else
		if (m2 == 0 || m2 == 1 || m2 == 2) && m1 != 1 && d_l1_m1_2 == 0 {
			Class::TypeA4
		} else
		if m1 == 1 && m2 == 0 && m3 <= 3 && d_l1_m3 != 0 {
			Class::TypeA5
		} else
		if m1 == 1 && m2 == 0 && m3 <= 2 && d_l1_m3 == 0 {
			Class::TypeA6
		} else
		if m1 == 1 && m2 <= 2 && m3 >= 4 && d_l1_m3 != 0 {
			Class::TypeB1
		} else
		if m1 == 1 && m2 <= 2 && m3 >= 3 && d_l1_m3 == 0 {
			Class::TypeB2
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 0 && m3 == 1) && l1 == 0 {
			Class::TypeB3
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 2 && m3 == 3) && l1 == 0 {
			Class::TypeB4
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 0 && m3 == 1 && m3 == 2) && l1 != 0 {
			Class::TypeB5
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && m3 == 3 && d_l1_3 != 0 {
			Class::TypeB6
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && m3 == 3 && l1 == 3 {
			Class::TypeB7
		} else {
			panic!("unclassified number {} (len: {}, m1: {}, m2: {}, m3: {}, l1: {})",
					self.as_u128(), self.digs.len(),
					m1, m2, m3, l1);
		}
	}

	/// numbers are special if the length is even and one of the middle digits is 0
	/// as defined in Definition 2.1 (Section 2.3)
	fn is_special(&self) -> bool {
		let l = self.digs.len();
		// odd length numbers and small numbers are not special
		if l % 2 != 0 || l < 7 { return false; }
		let m = l / 2;
		self.most_sig(m as usize) == 0 || self.least_sig(m as usize) == 0
	}
}

/// a trait for dealing with palindromes
trait Palindrome {
	/// the length of a palindrome
	fn len(&self) -> usize;
	/// index into the palindrome for the @nth digit
	/// - this verifies both most- and least- significant @nth digit are equal
	fn digit(&self, nth: usize) -> Digit;
	/// mutate the palindrome with the passed digit
	/// - this will change both most- and least- significant @nth digit
	/// - and then verify that they are both equal
	fn set_digit(&mut self, nth: usize, value: Digit);
}

/// an impl of Palindrome which wraps Number
struct Pal(Number);
impl Pal {
	/// create space of specific base for a palindrome to be constructed
	fn new(len: usize, base: Digit) -> Pal { Pal(Number{ digs: vec![0; len], base, }) }
	/// shorten the palindrome by one digit
	/// FIXME: unsound: only use on empty palindrome
	fn drop_digit(&mut self) {
		// TODO: drop middle digit to preserve palindrome status which should
		// - remove the unique digit of an odd-length palindrome
		// - remove one of the pair of digits in an even-length palindrome
		self.0.digs.pop().expect("drop digit failed");
		// TODO: verify still a palindrome
	}
}

impl Palindrome for Pal {
	fn len(&self) -> usize { self.0.digs.len() }
	fn digit(&self, nth: usize) -> Digit {
		assert!(nth < self.len(), "index out of bounds");
		assert!(self.0.least_sig(nth) == self.0.most_sig(nth), "invalid number");
		self.0.least_sig(nth)
	}
	fn set_digit(&mut self, nth: usize, value: Digit) {
		assert!(nth < self.len(), "index out of bounds");
		assert!(value < self.0.base, "value > base");
		let len = self.len();
		// set most significant digit
		self.0.digs[len - nth] = value;
		// set least significant digit
		self.0.digs[nth - 1] = value;
		assert!(self.digit(nth) == value, "value is set");
	}
}

fn main() {
	let arg = std::env::args().nth(1)
				.expect("Please provide number to decompose into palindromes");
	let num : u128 = arg.parse()
				.expect("Please provide positive number to decompose into palindromes");
	let digits = Number::from(num, BASE);
    println!("{:?}", digits);
    println!("Type: {:?}, special?: {}", digits.classify(), digits.is_special());
}

#[test]
fn test_classify_cover() {
	const X : u128 = 1_000_000;
	for n in 0 .. 100*X {
		let digits = Number::from(n, BASE);
		let covers = digits.classify();
		if n < X { assert!(covers == Class::Small, "not Small: {} ", digits.as_u128()); }
		if n % X == 0 {
			println!(".");
		}
	}
}
