/// This macro calculates the symbolic pattern char the represents the specified char.
/// Returns a char.
///
/// # Arguments
///
/// * `c: char` - The char in the entity to convert to a symbolic pattern char.</br>
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate test_data_generation;
/// # fn main() {
/// 	//let symbol: char = symbolize_char!(&'A');
///     //assert_eq!('V', symbol);
/// # }
/// ```
#[macro_export]
macro_rules! symbolize_char {
    ( $c:ident ) => {
        {
            let mut pattern = Pattern {
    			size: 0,
                regex_symbols: PatternPlaceholder::new(),
    			regex_consonant_upper: Regex::new(r"[B-DF-HJ-NP-TV-Z]").unwrap(),
    			regex_consonant_lower: Regex::new(r"[b-df-hj-np-tv-z]").unwrap(),
    			regex_vowel_upper: Regex::new(r"[A|E|I|O|U]").unwrap(),
    			regex_vowel_lower: Regex::new(r"[a|e|i|o|u]").unwrap(),
    			regex_numeric: Regex::new(r"[0-9]").unwrap(),
    			regex_punctuation: Regex::new(r"[.,\\/#!$%\\^&\\*;:{}=\\-_`~()\\?]").unwrap(),
    			regex_space: Regex::new(r"[\s]").unwrap(),
    		};

            // if you have to escape regex special characters: &*regex::escape(&*$c.to_string())
    		let mut symbol = pattern.regex_symbols.get("Unknown");
    		let mut found = false;

    		if !found && pattern.regex_consonant_upper.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("ConsonantUpper");
    			found = true;
    		}

    		if !found && pattern.regex_consonant_lower.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("ConsonantLower");
    			found = true;
    		}

    		if !found && pattern.regex_vowel_upper.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("VowelUpper");
    			found = true;
    		}

    		if !found && pattern.regex_vowel_lower.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("VowelLower");
    			found = true;
    		}

    		if !found && pattern.regex_numeric.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("Numeric");
    			found = true;
    		}

    		if !found && pattern.regex_space.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("WhiteSpace");
    			found = true;
    		}

    		if !found && pattern.regex_punctuation.is_match(&$c.to_string()) {
    			symbol = pattern.regex_symbols.get("Punctuation");
    			found = true;
    		}

    		// if not matched, then use "Unknown" placeholder symbol
    		if !found {
    			symbol = pattern.regex_symbols.get("Unknown");
    		}

    		symbol
        }
    };
}

/// This macro converts a char in an entity (&str) based on the index specified into a Fact</br>
///
/// # Arguments
///
/// * `entity: String` - The textual str of the value to anaylze.</br>
/// * `idx: u32` - The index that specifies the position of the char in the entity to convert to a Fact.</br>
///
/// # Example
///
/// ```/// # #[macro_use]
/// # extern crate test_data_generation;
/// # fn main() {
///		//let fact = factualize_entity!("Word",0);
///     // will return a Fact that represents the char `W`
/// # }
/// ```
macro_rules! factualize_entity {
    ( $entity:ident, $idx:ident ) => {
        {
            let c = $entity.chars().nth($idx as usize).unwrap();
            let pp = symbolize_char!(c);
            let pk = if $idx > 0 {$entity.chars().nth($idx as usize -1)} else {None};
            let nk = if $idx < $entity.len() as u32 -1 {$entity.chars().nth($idx as usize +1)} else {None};
            let sw = if $idx == 0 {1} else {0};
            let ew = if $idx == $entity.len() as u32 -1 {1} else {0};

            let mut fact = Fact::new(c,pp,sw,ew,$idx);

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
    }
}
