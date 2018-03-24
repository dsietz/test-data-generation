extern crate test_data_generation;

use test_data_generation::profile;

#[cfg(test)]
mod tests {
	use profile::fact::Fact;
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn new_fact(){
        //fact created for the character 'r' in the string "word"
    	let fact =  Fact::new('r','c',0,0,2);
    }
    #[test]
    fn new_fact_from_serialized(){
    	let serialized = "{\"key\":\"r\",\"prior_key\":null,\"next_key\":null,\"pattern_placeholder\":\"c\",\"starts_with\":0,\"ends_with\":0,\"index_offset\":2}";
    	let fact = Fact::from_serialized(&serialized);
    	assert_eq!(fact.pattern_placeholder, 'c');
    }
    
    #[test]
    // ensure A Fact can be exported (to be archived) as JSON
    fn serialize(){
        //fact created for the character 'r' in the string "word"
    	let mut fact =  Fact::new('r','c',0,0,2);
    	println!("serialized = {}", fact.serialize());
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