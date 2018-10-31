//! program for calculating the three palindromic numbers to sum into the number given as an argument

mod digit;
use digit::*;

mod classes;
use classes::Class;

mod numbers;
use numbers::Number;

mod palindromes;
use palindromes::*;

impl Number {
	fn classify(&self) -> (Class, impl Palindrome, impl Palindrome, impl Palindrome) {
		let len = self.digs.len();
		if len < 7 {
			return ( Class::Small, Pal::new(0, self.base), Pal::new(0, self.base), Pal::new(0, self.base) );
		}
		// most significant 3 digits
		let m1 = self.most_sig(1);
		let m2 = self.most_sig(2);
		let m3 = self.most_sig(3);
		// least significant digit
		let l1 = self.least_sig(1);
		// D() calculations
		let d_l1_m1_m2_1 = self.big_dee(&[l1, m1, m2, -1isize as Digit]);
		let d_l1_m1_2 = self.big_dee(&[l1, m1, -2isize as Digit]);
		let d_l1_m3 = self.big_dee(&[l1, m3]);
		let d_l1_3 = self.big_dee(&[l1, 3]);
		// three palindromes
		let mut p1 = Pal::new(len, self.base);
		let mut p2 = Pal::new(len, self.base);
		let mut p3 = Pal::new(len, self.base);
		// type A numbers
		(
		if !(m2 == 0 || m2 == 1 || m2 == 2) && d_l1_m1_m2_1 != 0 {
			p1.set_digit(1, m1);
			p2.drop_digit();
			p2.set_digit(1, m2 - 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, d_l1_m1_m2_1 as Digit);
			Class::TypeA1
		} else
		if !(m2 == 0 || m2 == 1 || m2 == 2) && d_l1_m1_m2_1 == 0 {
			p1.set_digit(1, m1);
			p2.drop_digit();
			p2.set_digit(1, m2 - 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, 1);
			Class::TypeA2
		} else
		if (m2 == 0 || m2 == 1 || m2 == 2) && m1 != 1 && d_l1_m1_2 != 0 {
			p1.set_digit(1, m1 - 1);
			p2.drop_digit();
			p2.set_digit(1, self.base - 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, d_l1_m1_2 as Digit);
			Class::TypeA3
		} else
		if (m2 == 0 || m2 == 1 || m2 == 2) && m1 != 1 && d_l1_m1_2 == 0 {
			p1.set_digit(1, m1 - 1);
			p2.drop_digit();
			p2.set_digit(1, self.base - 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, 1);
			Class::TypeA4
		} else
		if m1 == 1 && m2 == 0 && m3 <= 3 && d_l1_m3 != 0 {
			p1.drop_digit();
			p1.set_digit(1, self.base - 1);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, m3 + 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, d_l1_m3 as Digit);
			Class::TypeA5
		} else
		if m1 == 1 && m2 == 0 && m3 <= 2 && d_l1_m3 == 0 {
			p1.drop_digit();
			p1.set_digit(1, self.base - 1);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, m3 + 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, self.base - 1);
			Class::TypeA6
		} else
		if m1 == 1 && m2 <= 2 && m3 >= 4 && d_l1_m3 != 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, m3 - 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, d_l1_m3 as Digit);
			Class::TypeB1
		} else
		if m1 == 1 && m2 <= 2 && m3 >= 3 && d_l1_m3 == 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, m3 - 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, 1);
			Class::TypeB2
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 0 || m3 == 1) && l1 == 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2 - 1);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, self.base - 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, 1);
			Class::TypeB3
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 2 || m3 == 3) && l1 == 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, self.base - 2);
			Class::TypeB4
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && (m3 == 0 || m3 == 1 || m3 == 2) && l1 != 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2 - 1);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, self.base - 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, l1);
			Class::TypeB5
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && m3 == 3 && d_l1_3 != 0 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, 2);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, d_l1_3 as Digit);
			Class::TypeB6
		} else
		if m1 == 1 && (m2 == 1 || m2 == 2) && m3 == 3 && l1 == 3 {
			p1.set_digit(1, 1);
			p1.set_digit(2, m2);
			p2.drop_digit();
			p2.drop_digit();
			p2.set_digit(1, 1);
			p3.drop_digit();
			p3.drop_digit();
			p3.drop_digit();
			p3.set_digit(1, 1);
			Class::TypeB7
		} else {
			panic!("unclassified number {} (len: {}, m1: {}, m2: {}, m3: {}, l1: {})",
					self.as_u128(), self.digs.len(),
					m1, m2, m3, l1);
		}
		, p1, p2, p3 )

		// TODO: digit of p1 below m3 is always {0,1,2,3} ?
	}
}


fn main() {
	let arg = std::env::args().nth(1)
				.expect("Please provide number to decompose into palindromes");
	let num : u128 = arg.parse()
				.expect("Please provide positive number to decompose into palindromes");
	let digits = Number::from(num, BASE);
		println!("{:?}", digits);
		println!("Type: {:?}, special?: {}", digits.classify().0, digits.is_special());
}

#[cfg(test)]
mod tests{
	use numbers::Number;
	use classes::Class;
	use digit::BASE;
	const SMALL_NUMS : u128 = 1_000_000;
	#[test]
	fn classify_1234567() {
		let value = 1234567;
		let digits = Number::from(value, BASE);
		let covers = digits.classify().0;
		assert_eq!(covers, Class::TypeB6);
	}
	#[test]
	fn check_special() {
		let digits = Number::from(12300456, BASE);
		assert!(digits.is_special());
		let covers = digits.classify().0;
		assert_eq!(covers, Class::TypeB6);

		let digits = Number::from(45671230812390, BASE);
		assert!(digits.is_special());
		let covers = digits.classify().0;
		assert_eq!(covers, Class::TypeA1);

		let digits = Number::from(42471202812393, BASE);
		assert!(digits.is_special());
		let covers = digits.classify().0;
		assert_eq!(covers, Class::TypeA3);

		let digits = Number::from(42471252812393, BASE);
		assert!(!digits.is_special());
		let covers = digits.classify().0;
		assert_eq!(covers, Class::TypeA3);
	}
	#[test]
	fn classify_cover_small() {
		for n in 0 .. SMALL_NUMS {
			let digits = Number::from(n, BASE);
			let covers = digits.classify().0;
			assert_eq!(covers, Class::Small);
		}
	}
	#[test]
	fn classify_cover_x5() {
		let x = 5 * SMALL_NUMS;
		for n in x .. x + SMALL_NUMS {
			let digits = Number::from(n, BASE);
			let _covers = digits.classify().0;
		}
	}
	#[test]
	fn classify_cover_x420_sparse() {
		for n in SMALL_NUMS .. 2 * SMALL_NUMS {
			let digits = Number::from(n * 420, BASE);
			let _covers = digits.classify().0;
			if n % SMALL_NUMS == 0 {
				println!(".");
			}
		}
	}
}
