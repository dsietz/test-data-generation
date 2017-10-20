use test_data_generator::configs::Configs;
use test_data_generator::profile::profile::{Profile};

pub struct DataSampleParser{
	pub issues: bool,
	cfg: Configs,
}

impl DataSampleParser {
	//constructor
	pub fn new(path: &'static str) -> DataSampleParser {
		DataSampleParser{
			issues: false,
            cfg: Configs::new(path),
		}
	}
	
	// methods		
	pub fn demo_date(&self) -> String{
		let mut profil =  Profile::new();

    	profil.analyze("01/04/2017");
    	profil.analyze("02/09/2017");
    	profil.analyze("03/13/2017");
    	profil.analyze("04/17/2017");
    	profil.analyze("05/22/2017");
    	profil.analyze("07/26/2017");
    	profil.analyze("08/30/2017");
    	profil.analyze("09/07/2017");
    	profil.analyze("10/11/2017");
    	profil.analyze("11/15/2017");
    	profil.analyze("12/21/2017");    	
    	profil.analyze("01/14/2016");
    	profil.analyze("02/19/2016");
    	profil.analyze("03/23/2016");
    	profil.analyze("04/27/2016");
    	profil.analyze("05/02/2016");
    	profil.analyze("07/16/2015");
    	profil.analyze("08/20/2015");
    	profil.analyze("09/17/2015");
    	profil.analyze("10/01/2014");
    	profil.analyze("11/25/2014");
    	profil.analyze("12/31/2018");
    	
    	profil.pre_generate();	
    	//profil.apply_facts("##p##p####".to_string())   
    	profil.generate() 	
	}
	
	pub fn demo_person_name(&self) -> String{
	    let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brian, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronnae"); 
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Blake");
    	profil.analyze("Conways, Sephen");
    	
    	profil.pre_generate();	
    	profil.generate()
	}
	
	pub fn running_with_issues(&self) -> &bool{
		&self.issues
	}
}