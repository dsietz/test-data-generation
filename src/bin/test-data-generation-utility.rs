///
/// Data Algorithm Creator 
///
/// test-data-generation-utility.exe --tool="data-generator" --log="./config/log4rs.yaml" --config="./config/tdg.yaml"
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
	let dsp = DataSampleParser::new_with("/config/tdg.yaml");
	
	println!("generate date:{}", dsp.demo_date());
	println!("generate person:{}", dsp.demo_person_name());
}