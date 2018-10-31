//! Digit handling

/// initial base
// TODO: allow customising thru --base arg
pub(crate) const BASE: u8 = 10;

/// storage type for each digit (must fit BASE)
pub(crate) type Digit = u8;

