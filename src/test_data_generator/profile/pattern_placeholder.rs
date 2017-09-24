use std::collections::BTreeMap;

// types
type PlaceholderMap  = BTreeMap<String, char>;

pub struct PatternPlaceholder{
	pub placeholder: PlaceholderMap,
}

///
/// Placeholder for identifying paterns
/// A = upper case alpha
/// a = lower case alpha
/// # = numberic
/// ~ = special regex character
/// S = white space
/// p = punctuation
///
/// initialize a default PlaceholderMap
fn init() -> PlaceholderMap{
	let symbols: [char; 9] = ['~','C','c','V','v','#','~','S','p'];
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