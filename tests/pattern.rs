extern crate test_data_generation;

use test_data_generation::profile;

#[cfg(test)]
mod tests {
	use profile::pattern::Pattern;
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn symbolize_char(){
    	let pattern =  Pattern::new();
    	 		
    	assert_eq!(pattern.symbolize_char(&'A'), 'V');
    }
        
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn pattern_analyze(){
    	let mut pattern =  Pattern::new();
    	let word = pattern.analyze("HELlo0?^@");    
    			
    	assert_eq!(word.0, "CVCcv#pp@");
    }
}