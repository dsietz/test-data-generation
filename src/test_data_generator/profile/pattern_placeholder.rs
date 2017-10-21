//! The `pattern_placeholder` module provides functionality to retrieve symbols that are used in defining a pattern.
//!
//! Here is the list of symbols that identify a type of character:</br>
//! @ = unknown</br>
//! C = upper case consonant</br>
//! c = lower case consonant</br>
//! V = upper case vowel</br>
//! v = lower case vowel</br>
//! \# = numeric digit</br>
//! ~ = special regex character</br>
//! S = white space</br>
//! p = punctuation</br>
//!
//! # Examples
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::test_data_generator::profile::pattern_placeholder::PatternPlaceholder;
//! 
//! fn main() {
//!     // return the symbol that represents an upper case vowel
//!		pub fn upper_vowel_symbol() -> char {
//!			let placeholder =  PatternPlaceholder::new();
//!    		placeholder.get(&"VowelUpper".to_string())
//!		}
//!
//!     // confirm that the symbol used to represent an upper case vowel is 'V' 
//! 	assert_eq!(upper_vowel_symbol(), 'V');
//! }
//! ```

use std::collections::BTreeMap;

// types
type PlaceholderMap  = BTreeMap<String, char>;

pub struct PatternPlaceholder{
	pub placeholder: PlaceholderMap,
}

/// initialize a default PlaceholderMap
fn init() -> PlaceholderMap{
	let symbols: [char; 9] = ['@','C','c','V','v','#','~','S','p'];
	let mut pm = PlaceholderMap::new();
	
	pm.insert("Unknown".to_string(),        symbols[0]);
	pm.insert("ConsonantUpper".to_string(), symbols[1]);
	pm.insert("ConsonantLower".to_string(), symbols[2]);
	pm.insert("VowelUpper".to_string(),     symbols[3]);
	pm.insert("VowelLower".to_string(),     symbols[4]);
	pm.insert("Numeric".to_string(),        symbols[5]);
	pm.insert("RegExSpcChar".to_string(),   symbols[6]);
	pm.insert("WhiteSpace".to_string(),     symbols[7]);
	pm.insert("Punctuation".to_string(),    symbols[8]);
	
	pm.clone()
}

impl PatternPlaceholder {
	//constructor
	pub fn new() -> PatternPlaceholder {	
		let ph = init();
		PatternPlaceholder{
			placeholder: ph,
		}
	}	
	
	pub fn get(&self, key: &str) -> char {
		*self.placeholder.get(key).unwrap()
	}
}