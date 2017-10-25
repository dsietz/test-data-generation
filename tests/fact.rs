extern crate test_data_generation;

use test_data_generation::profile;

#[cfg(test)]
mod tests {
	use profile::fact::Fact;
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn create_fact(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    }
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn set_next_key(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	fact.set_next_key('d');
    }
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn set_prior_key(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	fact.set_prior_key('o');
    }
}