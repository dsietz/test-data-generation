extern crate test_data_generation;

use test_data_generation::test_data_generator::{profile};


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {    
	use profile::pattern_placeholder::PatternPlaceholder;

	// shared functions
	pub fn upper_vowel_symbol() -> char {
		let placeholder =  PatternPlaceholder::new();
    	placeholder.get(&"VowelUpper".to_string())
	}

	// tests
    #[test]
    // ensure PatternPlaceholder has the correct symbols
    fn pattern_placeholder(){
    	assert_eq!(upper_vowel_symbol(), 'V');
    }
}