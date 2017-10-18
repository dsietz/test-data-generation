///
/// key                 = the char that the fact defines (.e.g: 'a', '1', '%', etc.)
/// prior_key           = the char that appears before (-1) the key in the entity
/// next_key            = the char that appears after (+1) the key in the entity
/// pattern_placeholder = the PatternPlaceholder symbol that represents the type of key  
/// starts_with         = indicates if the key is the first char in the entity (0=no, 1=yes)
/// ends_with           = indicates if the key is the last char in the entity (0=no, 1=yes)
/// index_offset        = indicates the number of positions from the index zero (where the char is located in the entity from the first position)
/// 

#[derive(Debug)]

pub struct Fact{
	pub key: char,
	pub	prior_key: Option<char>,
	pub	next_key: Option<char>,
	pub	pattern_placeholder: char,
	pub	starts_with: u32,
	pub	ends_with: u32,
	pub	index_offset: u32,
}

impl Fact {
	//constructor
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
	
	pub fn set_next_key(&mut self, nk: char) {
		self.next_key = Some(nk);
	}
	
	pub fn set_prior_key(&mut self, pk: char) {
		self.prior_key = Some(pk);
	}
}