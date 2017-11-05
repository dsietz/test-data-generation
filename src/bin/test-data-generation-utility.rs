///
/// Data Algorithm Creator 
///
/// test-data-generation-utility.exe --tool="data-generator" --log="../../src/bin/config/log4rs.yaml" --config="../../src/bin/config/tdg.yaml"
///

#[macro_use]
extern crate log;
extern crate test_data_generation;
extern crate log4rs;

use test_data_generation::{params, data_sample_parser};
use params::Params;
use data_sample_parser::DataSampleParser;
//use configs::Configs;

// This is the main function
fn main() {
	// Params object holds the arguments passed 
	let params =  Params::new();
	
	// setup logging
	log4rs::init_file(params.get_log_file(), Default::default()).unwrap();
	info!("Logging enabled...");
	
	// start up a Data Sample Parser
	//borrowed content error when passing in params.get_config_file() as parameter
	let mut dsp = DataSampleParser::new_with("/config/tdg.yaml");
	
	info!("Demo ...");
	println!("generate date:{}", dsp.demo_date());
	println!("generate person:{}", dsp.demo_person_name());
    
    info!("Analyzing CSV file ...");	
   	println!("reading csv file: {}", dsp.analyze_csv_file("C:/workspace/test-data-generation/tests/samples/sample-01.csv").unwrap());
   	
   	info!("Generating a first name based on the CSV file ...");	
   	println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
   	
   	println!("Generated data record: {:?}",dsp.generate_record());
}