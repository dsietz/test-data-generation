extern crate test_data_generation;

use test_data_generation::profile;

#[cfg(test)]
mod tests {
	use profile::pattern::Pattern;
	use profile::fact::Fact;

    #[test]
    // ensure Pattern is analyzing data into patterns
    fn symbolize_char(){
    	let pattern =  Pattern::new();

    	assert_eq!(pattern.symbolize_char(&'A'), 'V');
    }

	#[test]
    // ensure Pattern is analyzing data into patterns
    fn factualize(){
    	let mut pattern =  Pattern::new();
    	let mut fact1 = pattern.factualize("Word",1);
		let mut fact2 = Fact::new('o','v',0,0,1);
		fact2.set_prior_key('W');
		fact2.set_next_key('r');

    	assert_eq!(fact1.serialize(), fact2.serialize());
    }

    #[test]
    // ensure Pattern is analyzing data into patterns
    fn pattern_analyze(){
    	let mut pattern =  Pattern::new();
    	let word = pattern.analyze("HELlo0?^@");

    	assert_eq!(word.0, "CVCcv#pp@");
    }
}
