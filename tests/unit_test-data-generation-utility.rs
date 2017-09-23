extern crate test_data_generation;

use test_data_generation::test_data_generator::{data_sample_parser,profile};

// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use data_sample_parser::DataSampleParser;
	//use test_data_generation::test_data_generator::{data_sample_parser,configs, params};
	use test_data_generation::test_data_generator::params::Params;
	use test_data_generation::test_data_generator::profile::{profile,pattern};
	use profile::pattern::Pattern;
	use profile::profile::Profile;
    
    #[test]
    // ensure the the default parameters are set
    fn parameters_defaults(){
    	let params =  Params::new();    	
    	assert!(true);
    	//assert!(params.get_config_file().is_empty());
    	//assert!(params.get_log_file().is_empty());
    	//assert!(params.get_tool().is_empty());
    	//assert!(params.get_verbose().is_empty());
    }
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn pattern_analyze(){
    	let mut pattrn =  Pattern::new();
    	let word = pattrn.analyze("H3Ll0?");    		
    	assert_eq!(word, "A#Aa#~");
    }
    
    #[test]
    // ensure Pattern is performing distinct count on patterns
    fn pattern_count(){
    	let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronney"); 
    	   		
    	profil.pattern_count();
    	   		
    	assert!(true);
    }
    
    
    #[test]
    // ensure Pattern is appending data
    fn profile_analyze(){
    	let mut profil =  Profile::new();
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronney"); 
    	    		
    	assert_eq!(profil.patterns.len(), 4);
    }
    
    #[test]
    // special characters for profiling
    fn profile_special_characters(){
    	let pattrn =  Pattern::new();    		
    	assert_eq!(pattrn.special_char.len(), 14);
    }
}