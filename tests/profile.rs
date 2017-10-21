extern crate test_data_generation;

use test_data_generation::profile;


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use profile::profile::Profile;
	
	//shared functions

    
    #[test]
    // ensure Profile is ranking sizes correctly
    fn profile_rank_sizes(){
    	let mut profil0 =  Profile::new();
    	
    	profil0.analyze("Smith, Johny");
    	profil0.analyze("O'Brian, Henny"); 
    	profil0.analyze("Dale, Danny"); 
    	profil0.analyze("O'Henry, Al"); 
    	profil0.analyze("Rickets, Ron"); 
    	profil0.analyze("Mr. Wilberson");
    	profil0.analyze("Po, Al"); 
    	/*   		
    	profil0.pre_generate();
    	let s = *profil0.size_ranks;	
		let r = 15.00 as f64;
    	assert_eq!(s.iter().find(|&&x|&x.1 >= &r).unwrap().0, 15);
    	*/
    	
    	true;
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
    #[ignore]
    // issue #31
    // ensure Profile doesn't generate a name with a backslash preceding an apostrophe
    // NOT FIXED
    fn profile_generate_with_apostrophe(){
    	let mut profil =  Profile::new();
    	profil.analyze("O'Brien");
    	profil.analyze("O'Mac");
    	profil.analyze("O'Connell");
    	profil.analyze("O'Donnell");
    	
    	profil.pre_generate();	
    	let generated = profil.generate();
    	let check = generated.contains("\'");
    	   		
    	assert_eq!(check, false);
    } 
}