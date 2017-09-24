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
	use std::collections::BTreeMap;
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
    // ensure Profile is ranking patterns correctly
    fn profile_rank(){
    	let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, Johny");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Shale, Honey"); 
    	profil.analyze("Rickets, Ronney"); 
    	profil.analyze("Mr. Wilberson"); 
    	   		
    	let rnk = profil.rank();

    	assert_eq!(*rnk.get("AaaaapSAaaaa").unwrap(), 33.33333333333333 as f64);
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
}