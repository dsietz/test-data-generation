//! 
//! 
//! # Fact
//! The Fact object is a representation of a character based on its context within a data entity. 
//! Facts are created during the analyze process and then later used to generate data from the algorithm.
//! 
//! ## Example
//!
//! ```rust
//! extern crate test_data_generation;
//!
//! use test_data_generation::engine::Fact;
//!
//! fn main() {
//!     //fact created for the character 'r' in the string "word"
//!    	let mut fact =  Fact::new('r','c',0,0,2);
//!
//!     // set the char that appears after the 'r'
//!     fact.set_next_key('d');
//!
//!     // set the char that appears before the 'r'
//!     fact.set_prior_key('o');
//! }
//! ```
//! 
//! # PatternDefinition
//! The PatternDefinition provides functionality to retrieve symbols that are used in defining a pattern.
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
//! ## Example
//!
//! ```rust
//! extern crate test_data_generation;
//!
//! use test_data_generation::engine::PatternDefinition;
//! 
//! fn main() {
//! 	let pttrn_def = PatternDefinition::new();
//!     println!("Upper case vowel symbol: {:?}", pttrn_def.get(&"VowelUpper".to_string()));
//! }
//! ```

use regex::Regex;
use serde_json;
use std::collections::BTreeMap;

#[allow(dead_code)]
type PatternMap  = BTreeMap<String, char>;

#[derive(Serialize, Deserialize, Debug)]
/// Represents a Fact for a character in a sample data entity that has been analyzed
pub struct Fact{
	/// the char that the fact defines (.e.g: 'a', '1', '%', etc.)
	pub key: char,
	/// the char that appears before (-1) the key in the entity
	pub	prior_key: Option<char>,
	/// the char that appears after (+1) the key in the entity
	pub	next_key: Option<char>,
	/// the PatternPlaceholder symbol that represents the type of key
	pub	pattern_placeholder: char,
	/// indicates if the key is the first char in the entity (0=no, 1=yes)
	pub	starts_with: u32,
	/// indicates if the key is the last char in the entity (0=no, 1=yes)
	pub	ends_with: u32,
	/// indicates the number of positions from the index zero (where the char is located in the entity from the first position)
	pub	index_offset: u32,
}

impl Fact {
	/// Constructs a new Fact
	///
	/// # Arguments
	///
	/// * `k: char` - The char that the Fact represents (also known as the `key`).</br>
	/// * `pp: char` - The char that represents the patter placeholder for the key.</br>
	/// * `sw: u32` - Indicates is the key is the first char in the entity. (0=no, 1=yes)</br>
	/// * `ew: u32` - Indicates is the key is the last char in the entity. (0=no, 1=yes)</br>
	/// * `idx_off: u32` - The index that represents the postion of the key from the beginning of the entity (zero based).</br>
	///
	/// # Example
	///
	/// ```rust
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::Fact;
	///
	/// fn main() {
	/// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
	/// }
	/// ```
	pub fn new(k: char, pp: char, sw: u32, ew: u32, idx_off: u32 ) -> Fact {
		Fact{
			key: k,
			prior_key: None,
			next_key: None,
			pattern_placeholder: pp,
			starts_with: sw,
			ends_with: ew,
			index_offset: idx_off,
		}
	}

	/// Constructs a new Fact from a serialized (JSON) string of the Fact object. This is used when restoring from "archive"
	///
	/// # Arguments
	///
	/// * `serialized: &str` - The JSON string that represents the archived Fact object.</br>
	///
	/// # Example
	///
	/// ```rust
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::Fact;
	///
	/// fn main() {
	///		let serialized = "{\"key\":\"r\",\"prior_key\":null,\"next_key\":null,\"pattern_placeholder\":\"c\",\"starts_with\":0,\"ends_with\":0,\"index_offset\":2}";
    ///		let mut fact = Fact::from_serialized(&serialized);
    ///     fact.set_prior_key('a');
    ///		fact.set_next_key('e');
    ///
    ///		assert_eq!(fact.pattern_placeholder, 'c');
    /// }
    /// ```
	pub fn from_serialized(serialized: &str) -> Fact {
		serde_json::from_str(&serialized).unwrap()
	}

    /// This function converts the Fact to a serialize JSON string.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::engine::Fact;
    ///
    /// fn main() {
    /// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///
    ///     println!("{}", fact.serialize());
    ///     // {"key":"r","prior_key":null,"next_key":null,"pattern_placeholder":"c","starts_with":0,"ends_with":0,"index_offset":2}
    /// }
    ///
	pub fn serialize(&mut self) ->String {
		serde_json::to_string(&self).unwrap()
	}

    /// This function sets the next key attribute to the specified char.
    ///
    /// # Arguments
    ///
    /// * `nk: char` - The character that represents the next character in the entity
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::engine::Fact;
    ///
    /// fn main() {
    /// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///     fact.set_next_key('d');
    /// }
    ///
	pub fn set_next_key(&mut self, nk: char) {
		self.next_key = Some(nk);
	}

    /// This function sets the prior key attribute to the specified char.
    ///
    /// # Arguments
    ///
    /// * `pk: char` - The character that represents the prior character in the entity
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::engine::Fact;
    ///
    /// fn main() {
    /// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///     fact.set_prior_key('o');
    /// }
    ///
	pub fn set_prior_key(&mut self, pk: char) {
		self.prior_key = Some(pk);
	}
}

/// Represents a symbolic pattern of an entity (String)
struct Pattern {
	/// The regex rule used to find upper case consonants
	regex_consonant_upper: Regex,
	/// The regex rule used to find lower case consonants
	regex_consonant_lower: Regex,
	/// The regex rule used to find upper case vowels
	regex_vowel_upper: Regex,
	/// The regex rule used to find lower case vowels
	regex_vowel_lower: Regex,
	/// The regex rule used to find numeric digits
	regex_numeric: Regex,
	/// The regex rule used to find punctuation
	regex_punctuation: Regex,
	/// The regex rule used to find white spaces
	regex_space: Regex,
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            regex_consonant_upper: Regex::new(r"[B-DF-HJ-NP-TV-Z]").unwrap(),
            regex_consonant_lower: Regex::new(r"[b-df-hj-np-tv-z]").unwrap(),
            regex_vowel_upper: Regex::new(r"[A|E|I|O|U]").unwrap(),
            regex_vowel_lower: Regex::new(r"[a|e|i|o|u]").unwrap(),
            regex_numeric: Regex::new(r"[0-9]").unwrap(),
            regex_punctuation: Regex::new(r"[.,\\/#!$%\\^&\\*;:{}=\\-_`~()\\?]").unwrap(),
            regex_space: Regex::new(r"[\s]").unwrap(),
        }
    }
}

/// Represents the object managing all the symbols used in pattern definitions
pub struct PatternDefinition {
    pattern_map: PatternMap,
    pattern: Pattern,
}

impl PatternDefinition {
    /// Constructs a new PatternDefinition
	/// 
	/// # Example
	/// 
	/// ```rust
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::PatternDefinition;
	///	
	/// fn main() {
	/// 	let pttrn_def = PatternDefinition::new();
	/// }
	/// ```
	pub fn new() -> PatternDefinition {	
        let symbols: [char; 9] = ['@','C','c','V','v','#','~','S','p'];
        let mut pttrn_def = PatternMap::new();

        pttrn_def.insert("Unknown".to_string(),        symbols[0]);
	    pttrn_def.insert("ConsonantUpper".to_string(), symbols[1]);
	    pttrn_def.insert("ConsonantLower".to_string(), symbols[2]);
	    pttrn_def.insert("VowelUpper".to_string(),     symbols[3]);
	    pttrn_def.insert("VowelLower".to_string(),     symbols[4]);
	    pttrn_def.insert("Numeric".to_string(),        symbols[5]);
	    pttrn_def.insert("RegExSpcChar".to_string(),   symbols[6]);
	    pttrn_def.insert("WhiteSpace".to_string(),     symbols[7]);
        pttrn_def.insert("Punctuation".to_string(),    symbols[8]);

		PatternDefinition{
            pattern_map: pttrn_def,
            pattern: Pattern::default(),
		}
    }	

    /// This function converts an entity (&str) into a tuplet (String, Vec<Fact>)</br>
	///
	/// # Arguments
	///
	/// * `entity: String` - The textual str of the value to anaylze.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::PatternDefinition;
	///
	/// fn main() {
	///		let mut pttrn_def = PatternDefinition::new();
	///
	/// 	assert_eq!(pttrn_def.analyze("Hello World").0, "CvccvSCvccc");
	/// }
	/// ```
	pub fn analyze(&mut self, entity: &str) -> (String, Vec<Fact>) {
		// record the length of the passed value
		//self.size = entity.len() as u32;

		// String to hold the pattern
		let mut pttrn = String::new();

		// Vec to hold all the Facts to be returned
		let mut facts = Vec::new();

		// record the pattern of the passed value
		for (i, _c) in entity.chars().enumerate() {
			//let fact = self.factualize(&entity, i as u32);
			let idx: u32 = i as u32;
			let fact = self.factualize(entity, idx);
			pttrn.push_str(&*fact.pattern_placeholder.to_string());
			facts.push(fact);
		}
		
		(pttrn, facts)
	}

    /// This function converts a char in an entity (&str) based on the index specified into a Fact</br>
	///
	/// # Arguments
	///
	/// * `entity: String` - The textual str of the value to anaylze.</br>
	/// * `idx: u32` - The index that specifies the position of the char in the entity to convert to a Fact.</br>
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::PatternDefinition;
	///
	/// fn main() {
	///		let mut pttrn_def = PatternDefinition::new();
	///		let fact = pttrn_def.factualize("Word",0);
	///     // will return a Fact that represents the char `W`
	/// }
	/// ```
	pub fn factualize(&mut self, entity: &str, idx: u32) -> Fact {
		let c = entity.chars().nth(idx as usize).unwrap();
		let pp = self.symbolize_char(c);
		let pk = if idx > 0 {entity.chars().nth(idx as usize -1)} else {None};
		let nk = if idx < entity.len() as u32 -1 {entity.chars().nth(idx as usize +1)} else {None};
		let sw = if idx == 0 {1} else {0};
		let ew = if idx == entity.len() as u32 -1 {1} else {0};

		let mut fact = Fact::new(c,pp,sw,ew,idx);

		// only if there is a next key
		if nk.is_some() {
			&fact.set_next_key(nk.unwrap());
		}

		// only if there is a prior key
		if pk.is_some() {
			&fact.set_prior_key(pk.unwrap());
		}

		fact
	}
    
    /// This function returns a pattern symbol that represents the type of character 
	/// 
	/// # Example
	///
	/// ```rust
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::PatternDefinition;
	///	
	/// fn main() {
	/// 	let pttrn_def = PatternDefinition::new();
	///     println!("Upper case vowel symbol: {:?}", pttrn_def.get(&"VowelUpper".to_string()));
	/// }
	/// ```
	pub fn get(&self, key: &str) -> char {
		*self.pattern_map.get(key).unwrap()
    }
    
    /// This function converts a char into a pattern symbol
	///
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::engine::PatternDefinition;
	///
	/// fn main() {
	/// 	let pttrn_def = PatternDefinition::new();
	/// 	println!("The pattern symbol for 'A' is {:?}", pttrn_def.symbolize_char('A'));
	///     // The pattern symbol for 'A' is V
	/// }
	/// ```
    pub fn symbolize_char(&self, c: char) -> char {
        // if you have to escape regex special characters: &*regex::escape(&*$c.to_string())
    	let mut symbol = self.pattern_map.get("Unknown");
        let mut found = false;
        
        if !found && self.pattern.regex_consonant_upper.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("ConsonantUpper");
            found = true;
        }

        if !found && self.pattern.regex_consonant_lower.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("ConsonantLower");
            found = true;
        }

        if !found && self.pattern.regex_vowel_upper.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("VowelUpper");
            found = true;
        }

        if !found && self.pattern.regex_vowel_lower.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("VowelLower");
            found = true;
        }

        if !found && self.pattern.regex_numeric.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("Numeric");
            found = true;
        }

        if !found && self.pattern.regex_space.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("WhiteSpace");
            found = true;
        }

        if !found && self.pattern.regex_punctuation.is_match(&c.to_string()) {
            symbol = self.pattern_map.get("Punctuation");
            found = true;
        }

        // if not matched, then use "Unknown" placeholder symbol
        if !found {
            symbol = self.pattern_map.get("Unknown");
        }

        *symbol.unwrap()
    }
}


// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_new(){
        //fact created for the character 'r' in the string "word"
    	let _fact =  Fact::new('r','c',0,0,2);
    	
    	assert!(true);
    }
    
    #[test]
    fn test_fact_new_from_serialized(){
    	let serialized = "{\"key\":\"r\",\"prior_key\":null,\"next_key\":null,\"pattern_placeholder\":\"c\",\"starts_with\":0,\"ends_with\":0,\"index_offset\":2}";
    	let fact = Fact::from_serialized(&serialized);
    	assert_eq!(fact.pattern_placeholder, 'c');
    }
    
    #[test]
    fn test_fact_serialize(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	let serialized = fact.serialize();
    	
		assert_eq!(serialized,"{\"key\":\"r\",\"prior_key\":null,\"next_key\":null,\"pattern_placeholder\":\"c\",\"starts_with\":0,\"ends_with\":0,\"index_offset\":2}");
    }
    
    #[test]
    fn test_fact_set_next_key(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	fact.set_next_key('d');
    }
    
    #[test]
    fn test_fact_set_prior_key(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	fact.set_prior_key('o');
    }

    #[test]
    fn test_pattern_definition_new() {
        let pttrn_def = PatternDefinition::new();
        assert_eq!(pttrn_def.get("VowelUpper"), 'V');
    }

    #[test]
    fn test_pattern_definition_symbolize_char(){
    	let pttrn_def = PatternDefinition::new();

    	assert_eq!(pttrn_def.symbolize_char('A'), 'V');
    }

    #[test]
    fn test_pattern_definition_factualize(){
    	let mut pttrn_def = PatternDefinition::new();
    	let mut fact1 = pttrn_def.factualize("Word",1);
		let mut fact2 = Fact::new('o','v',0,0,1);
		fact2.set_prior_key('W');
		fact2.set_next_key('r');

    	assert_eq!(fact1.serialize(), fact2.serialize());
    }

    #[test]
    fn test_pattern_definition_analyze(){
    	let mut pttrn_def = PatternDefinition::new();
    	let word = pttrn_def.analyze("HELlo0?^@");

        assert_eq!(word.0, "CVCcv#pp@");
        assert_eq!(word.1.len(), 9);
    }

}