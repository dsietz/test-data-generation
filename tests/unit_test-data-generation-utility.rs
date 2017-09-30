extern crate test_data_generation;

use test_data_generation::test_data_generator::{profile};


// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
	use test_data_generation::test_data_generator::profile::{pattern_placeholder};
	use profile::pattern::Pattern;
	use profile::profile::Profile;
	use profile::pattern_placeholder::PatternPlaceholder;
    
    #[test]
    // ensure Pattern is analyzing data into patterns
    fn pattern_analyze(){
    	let mut pattrn =  Pattern::new();
    	let word = pattrn.analyze("HELlo0?^@");    		
    	assert_eq!(word.0, "CVCcv#pp~");
    }
    
    #[test]
    // ensure PatternPlaceholder has the correct symbols
    fn pattern_placeholder(){
    	let placeholder =  PatternPlaceholder::new();
    	let symbols:[char; 9] = ['~','C','c','V','v','#','~','S','p'];	
    	assert_eq!(placeholder.get(&"VowelUpper".to_string()), symbols[3]);
    }
    
    #[test]
    // ensure Profile is ranking patterns correctly
    fn profile_rank_patterns(){
    	let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, Johny");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dhalg, Danny");
    	   		
    	let rnk = profil.rank_patterns();

    	assert_eq!(*rnk.get("CcvccpSCvccc").unwrap(),   66.66666666666666 as f64);
    	//assert_eq!(*rnk.get("V~CcvvcpSCvccc").unwrap(), 33.33333333333333 as f64);
    }
    
    #[test]
    // ensure Profile is ranking sizes correctly
    fn profile_rank_sizes(){
    	let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, Johny");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("O'Henry, Al"); 
    	profil.analyze("Rickets, Ron"); 
    	profil.analyze("Mr. Wilberson");
    	profil.analyze("Po, Al"); 
    	   		
    	let rnk = profil.rank_sizes();
    	let i   = 6 as u32;

    	assert_eq!(*rnk.get(&i).unwrap(), 14.285714285714285 as f64);
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
    // ensure Profile is generating correct test data
    fn profile_generate(){
    	let mut profil =  Profile::new();
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronney"); 
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Binkie");
    	profil.analyze("Conways, Steven");
    	
    	profil.pre_generate();	
    	    		
    	assert!(profil.generate());
    }
}