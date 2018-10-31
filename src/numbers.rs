//! impl the Number struct

use digit::*;

#[derive(Debug)]
pub(crate) struct Number {
	pub(crate) digs: Vec<Digit>,
	pub(crate) base: Digit,
}

impl Number {
	pub(crate) fn from(mut num: u128, base: Digit) -> Number {
		let b : u128 = base as u128;
		let mut digits : Vec<u8> = vec![];

		while num >= b {
			digits.push((num % b) as u8);
			num = num / b;
		}

		digits.push(num as u8);

		Number { digs: digits, base }
	}

	/// fetch the nth most significant digit
	pub(crate) fn most_sig(&self, nth: usize) -> Digit {
		assert!(self.digs.len() > nth, "out of range");
		self.digs[self.digs.len() - nth]
	}
	/// fetch the nth least significant digit
	pub(crate) fn least_sig(&self, nth: usize) -> Digit {
		assert!(self.digs.len() > nth, "out of range");
		self.digs[nth - 1]
	}

	pub(crate) fn as_u128(&self) -> u128 {
		let g = self.base as u128;
		self.digs
			.iter()
			.enumerate()
			.map(|(n, dig)| *dig as u128 * g.pow(n as u32) )
			.sum()
	}

	/// make @value into a valid digit
	pub(crate) fn big_dee(&self, values: &[Digit]) -> Digit {
		let mut value = values[0] as isize;
		for v in values[1..].iter() { value -= *v as isize; }
		let g = self.base as isize;
		while value < 0 { value += g; }
		(value % g) as Digit
	}

	/// numbers are special if the length is even and one of the middle digits is 0
	/// as defined in Definition 2.1 (Section 2.3)
	pub(crate) fn is_special(&self) -> bool {
		let l = self.digs.len();
		// odd length numbers and small numbers are not special
		if l % 2 != 0 || l < 7 { return false; }
		let m = l / 2;
		self.most_sig(m as usize) == 0 || self.least_sig(m as usize) == 0
	}
}


