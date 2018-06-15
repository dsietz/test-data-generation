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
//!
//! Save the algorithm ...
//!
//! Archive (export) the data sample parser object so that you can reuse the algorithm to generate test data at a later time.
//! This enables you to persist the algorithm without having to store the actual data sample that was used to create the algorithm - 
//! Which is important if you used 'real' data in your sample data.
//!  
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!
//! fn main() {
//! 	// analyze the dataset
//!		let mut dsp =  DataSampleParser::new();
//!
//!     assert_eq!(dsp.save("./tests/samples/empty-dsp").unwrap(), true);
//! }
//! ``` 
//!
//! Load an algorithm ...
//! 
//! Create a data sample parser from a previously saved (exported) archive file so you can generate test data based on the algorithm.</br>
//! *NOTE:* In this example, there was only one data point in the data smaple that was analyzed (the word 'OK'). This was intentional 
//! so the algorithm would be guaranteed to generate that same word. This was done ensure the assert_eq! returns true.  
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!	
//! fn main() {	
//!		let mut dsp = DataSampleParser::from_file("./tests/samples/sample-00-dsp");
//!
//!		assert_eq!(dsp.generate_record()[0], "OK".to_string());
//! }
//! ```  
//!
//! You can also generate a new csv file based on the data sample provided.
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::data_sample_parser::DataSampleParser;
//!	
//! fn main() {	
//!     let mut dsp =  DataSampleParser::new();  
//!    	
//!    	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
//!    	dsp.generate_csv(100, "./tests/samples/generated-01.csv").unwrap(); 
//! }
//! ```   
//!

use std::collections::BTreeMap;
use configs::Configs;
use profile::profile::{Profile};
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::result::Result;
use csv;
use std::error::Error;
use csv::WriterBuilder;
//use crossbeam;
use serde_json;
use oozie::similarity;
use std::collections::HashMap;


type ProfilesMap = BTreeMap<String, Profile>;

#[derive(Serialize, Deserialize, Debug)]
/// Represents the Parser for sample data to be used 
pub struct DataSampleParser{
	/// indicates if there were issues parsing and anlyzing the data sample
	pub issues: bool,
	/// Configs object that define the configuration settings
	cfg: Option<Configs>,
	/// List of Profiles objects identified by a unique profile name BTreeMap<String, Profile> 
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
	/// * `path: String - The full path name (including the file name and extension) to the configuration file.</br>
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
	///		let dsp = DataSampleParser::new_with("./config/tdg.yaml");
	/// }
	/// ```
	pub fn new_with(path: &'static str) -> DataSampleParser {
		DataSampleParser{
			issues: false,
            cfg: Some(Configs::new(path)),
            profiles: ProfilesMap::new(),
		}
	}
	
	/// Constructs a new DataSampleParser from an exported JSON file. This is used when restoring from "archive"
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {	
	///		let mut dsp = DataSampleParser::from_file("./tests/samples/sample-00-dsp");
    ///
    ///		assert_eq!(dsp.generate_record()[0], "OK".to_string());
	/// }    	
    /// ```	
	pub fn from_file(path: &'static str) -> DataSampleParser {
		// open the archive file	
		let mut file = match File::open(format!("{}.json",&path)) {
			Err(_e) => {
				error!("Could not open file {:?}", &path.to_string());
				panic!("Could not open file {:?}", &path.to_string());
			},
			Ok(f) => {
				info!("Successfully opened file {:?}", &path.to_string());
				f
			},
		};
		
		//read the archive file
		let mut serialized = String::new();
		match file.read_to_string(&mut serialized) {
			Err(e) => {
				error!("Could not read file {:?} because of {:?}", &path.to_string(), e.to_string());
				panic!("Could not read file {:?} because of {:?}", &path.to_string(), e.to_string());
			},
			Ok(s) => {
				info!("Successfully read file {:?}", &path.to_string());
				s
			},
		};		
	
		serde_json::from_str(&serialized).unwrap()
	}		
	
	/// This function analyzes sample data that is a csv formatted string and returns a boolean if successful.
	/// _NOTE:_ The csv properties are as follows:
	///       + headers are included as first line
	///       + double quote wrap text
	///       + double quote escapes is enabled			
	///       + delimiter is a comma
	///
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
	
		let mut rdr = csv::ReaderBuilder::new()
        	.has_headers(true)
        	.quote(b'"')
        	.double_quote(true)
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
		
		/*		
		// attempting to multithread the processing for updating the profiles		
		crossbeam::scope(|scope| {			  		
			//iterate through all the records
	    	for result in rdr.records() {
				scope.spawn(move || {	 
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
				});        	              
	    	}
		});	
		*/
		      
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
	
	/// This function creates a csv file of generated test data.
	/// Prior to calling this funciton, you need to call the analyze_csv_file() function.	
	/// _NOTE:_ The csv properties are as follows:
	///       + headers are included as first line
	///       + double quotes wrap text
	///       + double quote escapes is enabled		
	///       + delimiter is a comma
	///
	/// 
	/// # Arguments
	///
	/// * `row_count: u32` - The number of rows to generate.</br>
	/// * `path: &'static str` - The full path name where to save the csv file.</br>
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
    ///     dsp.generate_csv(100, "./tests/samples/generated-01.csv").unwrap();
	/// }
	/// ```
	pub fn generate_csv(&mut self, row_count: u32, path: &'static str) -> Result<(), Box<Error>> {
		info!("generating csv file {}", path);
		
		let mut wtr = try!(WriterBuilder::new()
		    .has_headers(true)
        	.quote(b'"')
        	.double_quote(true)
        	.delimiter(b',')
        	.from_path(path).map_err(|e| {
			error!("csv file {} couldn't be created!",path);
    		e.to_string()
		}));
		
		let headers = self.extract_headers();
		wtr.write_record(&headers)?;
		
		for r in 0..row_count {
			let mut record = Vec::new();
		
			for profile in self.profiles.iter_mut() {
				record.push(profile.1.generate());
			}
		
			wtr.write_record(&record)?;
		}
		
		wtr.flush()?;
		
		Ok(())
	}	
	
	/// This function returns a vector of header names
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
    ///     let headers = dsp.extract_headers();
    ///
    ///		assert_eq!(headers.len(), 2);
	/// }	
	pub fn extract_headers(&mut self) -> Vec<String> {
		let mut headers = vec!();
		
		for profile in self.profiles.iter_mut() {
			headers.push(profile.0.to_string());
		}
		
		headers
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
	
	/// This function saves (exports) the DataSampleParser to a JSON file. 
	/// This is useful when you wish to reuse the algorithm to generate more test data later.  
	/// 
	/// # Arguments
	///
	/// * `field: String` - The full path of the export file , excluding the file extension, (e.g.: "./test/data/custom-names").</br>	
	/// 
	/// #Errors
	/// If this function encounters any form of I/O or other error, an error variant will be returned. 
	/// Otherwise, the function returns Ok(true).</br>
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data_sample_parser::DataSampleParser;
	///	
	/// fn main() {
	/// 	// analyze the dataset
	///		let mut dsp =  DataSampleParser::new();
	///     dsp.analyze_csv_file("./tests/samples/sample-00.csv").unwrap();
	///
    ///     assert_eq!(dsp.save("./tests/samples/sample-00-dsp").unwrap(), true);
	/// }
	/// 	
	pub fn save(&mut self, path: &'static str) -> Result<(bool), io::Error>  {
		let dsp_json = serde_json::to_string(&self).unwrap();
					
		// Create the archive file	
		let mut file = match File::create(format!("{}.json",&path)) {
			Err(e) => {
				error!("Could not create file {:?}", &path.to_string());
				return Err(e);
			},
			Ok(f) => {
				info!("Successfully exported to {:?}", &path.to_string());
				f
			},
		};

		// Write the json string to file, returns io::Result<()>
    	match file.write_all(dsp_json.as_bytes()) {
        	Err(e) => {
            	error!("Could not write to file {}", &path.to_string());
            	return Err(e);
        	},
        	Ok(_) => {
        		info!("Successfully exported to {}", &path.to_string());
        	},
    	};	    	
 	
		Ok(true)
	}
	
	pub fn string_to_vector(&mut self, text: String) -> Vec<f64>{
		let vu8 = text.into_bytes();
		let mut vf64 = vec!();
		
		for b in &vu8 {
			vf64.push(*b as f64);
		}
		
		vf64
	}
	
	pub fn realistic_test(&mut self, generated_data: &'static str, sample_data: &'static str) -> Result<f64, Box<Error>> {
		//https://docs.rs/GSL/0.4.31/rgsl/statistics/fn.correlation.html
		//http://www.statisticshowto.com/probability-and-statistics/correlation-coefficient-formula/
		// pearson's chi square test
		// cosine similarity - http://blog.christianperone.com/2013/09/machine-learning-cosine-similarity-for-vector-space-models-part-iii/
		
		let mut str_gen = String::from(generated_data); 
		let mut str_smpl = String::from(sample_data); 
		
		while str_gen.len() < str_smpl.len() {
			str_gen.push(' ');
		}
		
		while str_smpl.len() < str_gen.len() {
			str_smpl.push(' ');
		}
		
		let gen_data = self.string_to_vector(str_gen);
		let smpl_data = self.string_to_vector(str_smpl);
		
		let mut gen_map: HashMap<usize, f64> = HashMap::new();
		let gen_sz = gen_data.len();
		for gd in gen_data {
			gen_map.insert(gen_sz, gd);
		}
		
		let mut smpl_map: HashMap<usize, f64> = HashMap::new();
		let smpl_sz = smpl_data.len();
		for sd in smpl_data {
			smpl_map.insert(smpl_sz, sd);
		}
		
		
		let cos = similarity::cosine(&gen_map, &smpl_map, gen_sz);
		println!("cosine simularity {:?}", cos);
		//let v = vec!(111 as f64, 101 as f64);
		//let avg_gen_data = statistical::mean(&gen_data);
		
		//println!("{}",avg_gen_data);
		//let corr = statistical::correlation(gen_data, 1 as usize, sam_data, 1 as usize, sam_data.len());
		//println!("the Correlation Coefficient is {}",avg_gen_data);
		
		Ok(1 as f64)
	}
}