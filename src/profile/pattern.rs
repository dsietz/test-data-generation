///
/// Regex Symbols placeholder
/// A = upper case alpha
/// a = lower case alpha
/// # = numberic
/// ~ = special regex character
/// S = white space
/// p = punctuation
///

use regex::Regex;
use profile::pattern_placeholder::PatternPlaceholder;
use profile::fact::Fact;

pub struct Pattern{
	pub size: u32,
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
	
	pub fn analyze(&mut self, entity: &str) -> (String, Vec<Fact>) {
		// record the length of the passed value
		self.size = entity.len() as u32;
		
		// String to hold the pattern
		let mut pttrn = String::new();
		
		// Vec to hold all the Facts to be returned
		let mut facts = Vec::new();
		
		// record the pattern of the passed value
		for (i, c) in entity.chars().enumerate() {
			let mut pk      = None;
			let mut nk      = None;
			let     pp      = self.parse_entity(&c);
			let mut sw      = 0;
			let mut ew      = 0;
			let     idx_off = i;	
		
			// first char in entity
			if i == 0 {
				sw = 1;			
			}
			
			// last char in entity
			if i == (self.size as usize)-1 {
				ew = 1;				
			}
			
			// not the first
			if i > 0 {
				pk = entity.chars().nth(i-1);
			}
			
			// not the last
			if i < (self.size as usize)-1 {
				nk = entity.chars().nth(i+1);
			}
			
			// store the Facts in a HashMap of HashMaps that will be evenly distributed 
			// so the MapReduce can be performed for multiple threads calculating when aggregating 
			// on the Facts
			println!("char is {:?}",c);
			let mut f = Fact::new(c,pp,sw,ew,(idx_off as u32));
			
			// only if there is a next key
			if nk.is_some() {
				&f.set_next_key(nk.unwrap());
			}
			
			// only if there is a prior key
			if pk.is_some() {
				&f.set_prior_key(pk.unwrap());
			}
			
			pttrn = [&pttrn, &*pp.to_string()].concat();		
			facts.push(f);
		}
		
		(pttrn, facts)
	}
}