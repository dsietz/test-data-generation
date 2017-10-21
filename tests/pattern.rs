extern crate test_data_generation;

use test_data_generation::profile;


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use profile::pattern::Pattern;
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn pattern_analyze(){
    	let mut pattrn =  Pattern::new();
    	let word = pattrn.analyze("HELlo0?^@");    		
    	assert_eq!(word.0, "CVCcv#pp@");
    }
}