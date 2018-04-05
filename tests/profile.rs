extern crate test_data_generation;

use test_data_generation::profile;

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use profile::profile::Profile; 
	
	#[test]
    fn save_profile(){
		let mut profile =  Profile::new();
		profile.analyze("Smith, John");
    	profile.analyze("O'Brian, Henny"); 
    	profile.analyze("Dale, Danny"); 
    	profile.analyze("Rickets, Ronney");
    
    	profile.pre_generate(); 
	
        assert_eq!(profile.save("./tests/samples/sample-00-profile").unwrap(), true);
	}
	
	#[test]
    fn new_profile_from_file(){
		let mut profile = Profile::from_file("./tests/samples/sample-00-profile");
    	profile.pre_generate();
    
    	assert!(profile.generate().len() > 0);
    }	
	
	#[test]
    fn new_profile_from_serialized(){
    	let serialized = "{\"patterns\":{\"VC\":1},\"pattern_total\":1,\"pattern_keys\":[\"VC\"],\"pattern_vals\":[1],\"pattern_percentages\":[],\"pattern_ranks\":[],\"sizes\":{\"2\":1},\"size_total\":1,\"size_ranks\":[],\"processors\":4,\"facts\":[[{\"key\":\"O\",\"prior_key\":null,\"next_key\":\"K\",\"pattern_placeholder\":\"V\",\"starts_with\":1,\"ends_with\":0,\"index_offset\":0}],[{\"key\":\"K\",\"prior_key\":\"O\",\"next_key\":null,\"pattern_placeholder\":\"C\",\"starts_with\":0,\"ends_with\":1,\"index_offset\":1}],[],[]]}";
    	let mut profile = Profile::from_serialized(&serialized);
    	profile.pre_generate();
    	
    	assert_eq!(profile.generate(), "OK");
    }
	
	#[test]
    // ensure logging is working in the crate
    fn logging_test(){
    	let mut profile =  Profile::new();
    	profile.reset_analyze();
    	    		
    	assert!(true);
    }    
    
    #[test]
    // ensure Profile is analyzing all the sample data points
    fn profile_analyze(){
    	let mut profil =  Profile::new();
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronney"); 
    	    		
    	assert_eq!(profil.patterns.len(), 4);
    }
    
    #[test]
    // ensure Profile is providing the correct pattern ranks after analyzing the sample data
    fn profile_pregenerate_patterns(){
    	let mut profil =  Profile::new();
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronnae"); 
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Blake");
    	profil.analyze("Conways, Sephen");
    	
    	profil.pre_generate();	
    	let test = [("CvccvccpSCvccvv".to_string(), 28.57142857142857 as f64), ("CcvccpSCvcc".to_string(), 42.857142857142854 as f64), ("CvccvccpSCvccvc".to_string(), 57.14285714285714 as f64), ("CvcvcccpSCcvcv".to_string(), 71.42857142857142 as f64), ("CvcvpSCvccc".to_string(), 85.7142857142857 as f64), ("V@CcvvcpSCvccc".to_string(), 99.99999999999997 as f64)];    		
    	    		
    	assert_eq!(profil.pattern_ranks, test);
    }

    #[test]
    // ensure Profile is providing the correct pattern ranks after analyzing the sample data
    fn profile_pregenerate_sizes(){
    	let mut profil =  Profile::new();

    	profil.analyze("Smith, Johny"); //12
    	profil.analyze("O'Brian, Hen"); //12 
    	profil.analyze("Dale, Danny");  //11
    	profil.analyze("O'Henry, Al");  //11
    	profil.analyze("Rickets, Ro");  //11
    	profil.analyze("Mr. Wilbers");  //11
    	profil.analyze("Po, Al");       //6  
    	
    	profil.pre_generate();	
    	let test = [(11, 57.14285714285714), (12, 85.71428571428571), (6, 100 as f64)];    		
    	    		
    	assert_eq!(profil.size_ranks, test);
    }
    
    #[test]
    // ensure Profile is able to find the facts that relate to a pattern
    fn profile_apply_facts_string(){
    	let mut profil =  Profile::new();
    	profil.analyze("First");
    	profil.analyze("Next");
    	profil.analyze("Last");
    	
    	profil.pre_generate();	
    	let generated = profil.apply_facts("Cvcc".to_string());
    	   		
    	assert_eq!(4, generated.len());
    } 
    
    #[test]
    // ensure Profile is able to find the facts that relate to a pattern
    // NOTE: Dates need work! e.g.: 00/15/0027
    fn profile_apply_facts_date(){
    	let mut profil =  Profile::new();
    	profil.analyze("01/13/2017");
    	profil.analyze("11/24/2017");
    	profil.analyze("08/05/2017");
    	
    	profil.pre_generate();	
    	let generated = profil.apply_facts("##p##p####".to_string());
    	   		
    	assert_eq!(10, generated.len());
    }       
        
    #[test]
    // ensure Profile is generating correct test data
    fn profile_generate(){
    	let mut profil =  Profile::new();
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronnae"); 
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Blake");
    	profil.analyze("Conways, Sephen");
    	
    	profil.pre_generate();	
    	    		
    	assert!(profil.generate().len() > 10);
    }
    
    #[test]
    // issue #31
    // ensure Profile doesn't generate a name with a backslash preceding an apostrophe
    fn profile_generate_with_apostrophe(){
    	let mut profil =  Profile::new();
    	profil.analyze("O'Brien");
    	
    	profil.pre_generate();	
    	let generated = profil.generate();
 		
    	assert_eq!(generated, "O'Brien");
    }
    
    #[test]
    // ensure a Profile can be exported (to be archived) as JSON
    fn serialize(){
		let mut profil =  Profile::new();
		
		// analyze the dataset
		profil.analyze("OK");
		
    	let serialized = profil.serialize();
    	assert_eq!(serialized, "{\"patterns\":{\"VC\":1},\"pattern_total\":1,\"pattern_keys\":[\"VC\"],\"pattern_vals\":[1],\"pattern_percentages\":[],\"pattern_ranks\":[],\"sizes\":{\"2\":1},\"size_total\":1,\"size_ranks\":[],\"processors\":4,\"facts\":[[{\"key\":\"O\",\"prior_key\":null,\"next_key\":\"K\",\"pattern_placeholder\":\"V\",\"starts_with\":1,\"ends_with\":0,\"index_offset\":0}],[{\"key\":\"K\",\"prior_key\":\"O\",\"next_key\":null,\"pattern_placeholder\":\"C\",\"starts_with\":0,\"ends_with\":1,\"index_offset\":1}],[],[]]}");
    }    
}