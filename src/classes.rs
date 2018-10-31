//! classifiaction fo numbers form Section 2.1 & 2.2


/// classification of numbers from Section 2.1 & 2.2
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum Class {
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

