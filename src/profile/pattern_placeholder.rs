//! The `pattern_placeholder` module provides functionality to retrieve symbols that are used in defining a pattern.
//!
//! Here is the list of symbols that identify a type of character:</br>
//! @ = unknown [Unknonw]</br>
//! C = upper case consonant [ConsonantUpper]</br>
//! c = lower case consonant [ConsonantLower]</br>
//! V = upper case vowel [VowelUpper]</br>
//! v = lower case vowel [VowelLower]</br>
//! \# = numeric digit [Numeric]</br>
//! ~ = special regex character [RegExSpcChar]</br>
//! S = white space [WhiteSpace]</br>
//! p = punctuation [Punctuation]</br>
//!
//! # Example
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::pattern_placeholder::PatternPlaceholder;
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

/// Represents a pattern placeholder
pub struct PatternPlaceholder{
	/// A PatternPlaceholder must have a PlaceholderMap that contains all the symbols used to categorize characters by type (e.g.: Upper Case Vowel)
	pub placeholder: PlaceholderMap,
}

/// initialize a default PlaceholderMap with the pattern symbols
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
	/// Constructs a new PatternPlaceholder
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::pattern_placeholder::PatternPlaceholder;
	///	
	/// fn main() {
	/// 	let placeholder = PatternPlaceholder::new();
	/// }
	/// ```
	pub fn new() -> PatternPlaceholder {	
		let ph = init();
		PatternPlaceholder{
			placeholder: ph,
		}
	}	
	
	/// This function returns a pattern symbol that represents the type of character 
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::pattern_placeholder::PatternPlaceholder;
	/// 
	/// fn main() {
	///		let placeholder =  PatternPlaceholder::new();
	/// 
	/// 	println!("Upper case vowel symbol: {:?}", placeholder.get(&"VowelUpper".to_string()));
	/// }
	/// ```	
	pub fn get(&self, key: &str) -> char {
		*self.placeholder.get(key).unwrap()
	}
}