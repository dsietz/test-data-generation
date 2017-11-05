//! The `data_sample_parser` module provides functionality to read sample data, parse and analyze it, 
//! so that test data can be generated based on profiles.
//!
//! # Examples
//!
//!
//! Generate some demo test data ...
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//! 
//! fn main() {
//!		// initalize a new DataSampelParser
//!		let dsp = DataSampleParser::new();
//!
//!		// generate some test data using the demo functions
//!		println!("generate date:{}", dsp.demo_date());
//!		println!("generate person:{}", dsp.demo_person_name());
//! }
//! ```

use std::collections::BTreeMap;
use configs::Configs;
use profile::profile::{Profile};
use std::fs::File;
use std::result::Result;
//use std::io::prelude::*;
//use csv_core::{Reader, ReadFieldResult};
use csv;
//use csv::Reader;

type ProfilesMap = BTreeMap<String, Profile>;

/// Represents the Parser for sample data to be used 
pub struct DataSampleParser{
	/// indicates if there were issues parsing and anlyzing the data sample
	pub issues: bool,
	/// Configs object that define the configuration settings
	cfg: Option<Configs>,
	/// List of Profiles objects identified by a unique profile name BTreeMap<String, Profile>
	// 
	profiles: ProfilesMap,
}

impl DataSampleParser {
	/// Constructs a new DataSampleParser
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	/// }
	/// ```
	pub fn new() -> DataSampleParser {
		
		DataSampleParser{
			issues: false,
            cfg: None,
            profiles: ProfilesMap::new(),
		}
	}
	
	/// Constructs a new DataSampleParser
	///
	/// # Arguments
	///
	/// * `path: &'static str` - The full path name (including the file name and extension) to the configuraiton file.</br>
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///	    // param: the path to the configuration  file
	///		let dsp = DataSampleParser::new_with("/config/tdg.yaml");
	/// }
	/// ```
	pub fn new_with(path: &'static str) -> DataSampleParser {
		DataSampleParser{
			issues: false,
            cfg: Some(Configs::new(path)),
            profiles: ProfilesMap::new(),
		}
	}
	
	/// This function analyzes sample data that is a csv formatted string and returns a boolean if successful.
	/// 
	/// # Arguments
	///
	/// * `path: &'static str` - The full path name of the csv formatted sample data file.</br>
	///
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///	
    /// 	assert_eq!(dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap(),1);
	/// }
	/// ```	
	pub fn analyze_csv_file(&mut self, path: &'static str) -> Result<i32, String>  {
		info!("Starting to analyzed the csv file {}",path);
    	
    	let file = try!(File::open(path).map_err(|e| {
			error!("csv file {} couldn't be opened!",path);
    		e.to_string()
		}));
		
		//let mut rdr = csv::Reader::from_reader(file);
		let mut rdr = csv::ReaderBuilder::new()
        	.has_headers(true)
        	.quote(b'"')
        	.delimiter(b',')
        	.from_reader(file);
        	
		//iterate through the headers
		for headers in rdr.headers() {
			for header in headers.iter() {
	        	//add a Profile to the list of profiles to represent the field (indexed using the header label)
	        	let p = Profile::new();
	        	self.profiles.insert(format!("{}",header), p);
	        }
		}
		
		//create a Vec from all the keys (headers) in the profiles list
		let profile_keys: Vec<_> = self.profiles.keys().cloned().collect();
		let mut rec_cnt: u16 = <u16>::min_value();
		
		debug!("CSV headers: {:?}",profile_keys);
		
		//iterate through all the records
	    for result in rdr.records() {
	        let record = result.expect("a CSV record");
	        
	        //keep a count of the number of records analyzed
	        rec_cnt = rec_cnt + 1;
	        
	        //iterate through all the fields
	        for (idx, field) in record.iter().enumerate() {
	        	// Print a debug version of the record.
	        	debug!("Field Index: {}, Field Value: {}", idx, field);
	        	
	        	//select the profile based on the field name (header) and analyze the field value
	        	self.profiles.get_mut(&profile_keys[idx]).unwrap().analyze(field);
	        }
	    }
	    
	    info!("Successfully analyzed the csv file {}", path);
		debug!("Analyzed {} records, {} fields", rec_cnt, self.profiles.len());
		
		//prepare the profiles for data generation
		self.profiles.iter_mut().for_each(|p|p.1.pre_generate());
		
		Ok(1)
	}
		
	/// This function generates date as strings using the a `demo` profile
	/// 
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	///
	///		// generate some test data using the demo functions
	///		println!("generate date:{}", dsp.demo_date());
	/// }
	/// ```	
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
	
	/// This function generates people's names as strings using the a `demo` profile
	/// 
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let dsp = DataSampleParser::new();
	///
	///		// generate some test data using the demo functions
	///		println!("generate date:{}", dsp.demo_person_name());
	/// }
	pub fn demo_person_name(&self) -> String{
	    let mut profil =  Profile::new();
    	
    	profil.analyze("Smith, John");
    	profil.analyze("O'Brien, Henny"); 
    	profil.analyze("Dale, Danny"); 
    	profil.analyze("Rickets, Ronnae"); 
    	profil.analyze("Richard, Richie");
    	profil.analyze("Roberts, Blake");
    	profil.analyze("Conways, Sephen");
    	
    	profil.pre_generate();	
    	profil.generate()
	}
	
	/// This function generates test data for the specified field name.
	/// 
	/// # Arguments
	///
	/// * `field: String` - The name of the field (e.g.: firstname) the represents the profile to use when generating the test data.</br>
	///
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///	
    /// 	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
    ///     println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
	/// }
	/// ```
	pub fn generate_by_field_name(&mut self, field: String) -> String {
		self.profiles.get_mut(&field).unwrap().generate().to_string()
	}
	
	/// This function Vec of generates test data fields.
	/// 
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///		let mut dsp = DataSampleParser::new();
    ///	
    /// 	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
    ///     println!("Generated data record: {:?}",dsp.generate_record());
	/// }
	/// ```
	pub fn generate_record(&mut self) -> Vec<String> {
		let mut record = Vec::new();
		
		for profile in self.profiles.iter_mut() {
			record.push(profile.1.generate().to_string());
		}
		
		record
	}
	
	/// This function returns a boolean that indicates if the data sample parsing had issues
	/// 
	/// # Example
	///
	/// ``` 
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	///		// initalize a new DataSampelParser
	///	    // param: the path to the configuration file is wrong
	///		let dsp = DataSampleParser::new_with("./target/debug/config/tdg.yaml");
	///
	///		// generate some test data using the demo functions
	///		assert_eq!(dsp.running_with_issues(), &false);
	/// }
	pub fn running_with_issues(&self) -> &bool{
		&self.issues
	}
}