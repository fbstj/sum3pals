//! palindrome numbes and wrapping around Number

use digit::*;
use numbers::Number;

/// a trait for dealing with palindromes
pub(crate) trait Palindrome {
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
pub(crate) struct Pal(Number);
impl Pal {
	/// create space of specific base for a palindrome to be constructed
	pub(crate) fn new(len: usize, base: Digit) -> Pal { Pal(Number{ digs: vec![0; len], base, }) }
	/// shorten the palindrome by one digit
	/// FIXME: unsound: only use on empty palindrome
	pub(crate) fn drop_digit(&mut self) {
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
