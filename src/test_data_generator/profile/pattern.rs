///
///
///
/// Regex Symbols placeholder
/// A = upper case alpha
/// a = lower case alpha
/// # = numberic
/// ~ = special regex character
/// S = white space
/// p = punctuation
///
///

use regex;
use regex::Regex;
use test_data_generator::profile::pattern_placeholder::PatternPlaceholder;
use test_data_generator::profile::fact::Fact;

pub struct Pattern{
	pub size: u32,
		reg_exp: String,
		regex_symbols: PatternPlaceholder,
		regex_consonant_upper: Regex,
		regex_consonant_lower: Regex,
		regex_vowel_upper: Regex,
		regex_vowel_lower: Regex,
		regex_numeric: Regex,
		regex_punctuation: Regex,
		regex_space: Regex,
}

impl Pattern {
	//constructor
	pub fn new() -> Pattern {
		Pattern{
			size: 0,
			reg_exp: String::from(""),
			regex_symbols: PatternPlaceholder::new(),
			regex_consonant_upper: Regex::new(r"[B-DF-HJ-NP-TV-Z]").unwrap(),
			regex_consonant_lower: Regex::new(r"[b-df-hj-np-tv-z]").unwrap(),
			regex_vowel_upper: Regex::new(r"[A|E|I|O|U]").unwrap(),
			regex_vowel_lower: Regex::new(r"[a|e|i|o|u]").unwrap(),
			regex_numeric: Regex::new(r"[0-9]").unwrap(),
			regex_punctuation: Regex::new(r"[.,\\/#!$%\\^&\\*;:{}=\\-_`~()\\?]").unwrap(),
			regex_space: Regex::new(r"[\s]").unwrap(),
		}
	}
	
	fn parse_entity(&self, c: &char) -> char{
		// if you have to escape regex special characters: &*regex::escape(&*&c.to_string())
		let mut x = self.regex_symbols.get("Unknown");
		let mut found = false;
			
		if !found && self.regex_consonant_upper.is_match(&c.to_string()) {
			x = self.regex_symbols.get("ConsonantUpper"); 
			found = true;
		}
			
		if !found && self.regex_consonant_lower.is_match(&c.to_string()) {
			x = self.regex_symbols.get("ConsonantLower"); 
			found = true;
		}
			
		if !found && self.regex_vowel_upper.is_match(&c.to_string()) {
			x = self.regex_symbols.get("VowelUpper");  
			found = true;
		}
			
		if !found && self.regex_vowel_lower.is_match(&c.to_string()) {
			x = self.regex_symbols.get("VowelLower");  
			found = true;
		}
			
		if !found && self.regex_numeric.is_match(&c.to_string()) {
			x = self.regex_symbols.get("Numeric");  
			found = true;
		}
			
		if !found && self.regex_space.is_match(&c.to_string()) {
			x = self.regex_symbols.get("WhiteSpace"); 
			found = true; 
		}
			
		if !found && self.regex_punctuation.is_match(&c.to_string()) {
			x = self.regex_symbols.get("Punctuation");  
			found = true;
		}
			
		// if not matched, then use "Unknown" placeholder symbol
		if !found {
			x = self.regex_symbols.get("Unknown");
		}
			
		x
	}
	
	pub fn analyze(&mut self, entity: &str) -> &String{
		// record the length of the passed value
		self.size = entity.len() as u32;
		
		// record the pattern of the passed value
		for c in entity.chars(){
			let pp = self.parse_entity(&c);
			// store the Facts in a HashMap of HashMaps that will be evenly distributed 
			// so the MapReduce can be performed for multiple threads calculating when aggregating 
			// on the Facts
			let f = Fact::new(c,self.regex_symbols.get("Unknown"),self.regex_symbols.get("Unknown"),pp,0,0,0);
			self.reg_exp = [&self.reg_exp, &*pp.to_string()].concat();		
		}		
		&self.reg_exp
	}
}