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

use regex::Regex;

pub struct Pattern{
	reg_exp: String,
	regex_symbols: [char; 6],
	regex_alpha_upper: Regex,
	regex_alpha_lower: Regex,
	regex_numeric: Regex,
	regex_punctuation: Regex,
	regex_special: Regex,
	regex_space: Regex,
}

impl Pattern {
	//constructor
	pub fn new() -> Pattern {
		Pattern{
			reg_exp: String::from(""),
			regex_symbols: ['A','a','#','~','S','p'],
			regex_alpha_upper: Regex::new(r"[A-Z]").unwrap(),
			regex_alpha_lower: Regex::new(r"[a-z]").unwrap(),
			regex_numeric: Regex::new(r"[0-9]").unwrap(),
			//regex_punctuation: Regex::new(r"[.,\/#!$%\^&\*;:{}=\-_`~()]").unwrap(),
			regex_punctuation: Regex::new(r"[.,\\/#!$%\\^&\\*;:{}=\\-_`~()]").unwrap(),
			regex_special: Regex::new(r"[\\\\\^\\$\\.\\|\\?\\*\\+\\(\\)\\[\\]\\{\\}]").unwrap(),
			regex_space: Regex::new(r"[\s]").unwrap(),
		}
	}
	
	pub fn analyze(&mut self, entity: &str) -> &String{
		for c in entity.chars(){
			if self.regex_alpha_upper.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[0].to_string()].concat(); 
			}
			
			if self.regex_alpha_lower.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[1].to_string()].concat(); 
			}
			
			if self.regex_numeric.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[2].to_string()].concat(); 
			}
			
			if self.regex_special.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[3].to_string()].concat(); 
			}
			
			if self.regex_space.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[4].to_string()].concat(); 
			}
			
			if self.regex_punctuation.is_match(&c.to_string()) {
				self.reg_exp = [&self.reg_exp, &*self.regex_symbols[5].to_string()].concat(); 
			}
		}
		
		&self.reg_exp
	}
}

//let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//assert!(re.is_match("2014-01-01"));
//
// scalar types: integers, floating-point numbers, booleans, and characters
// https://doc.rust-lang.org/regex/regex/index.html#ascii-character-classes
// https://regex101.com/
//
//
//