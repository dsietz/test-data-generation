/*
** Data Algorithm Creator 
*/
#[macro_use]
extern crate log;
extern crate test_data_generation;
extern crate log4rs;

use test_data_generation::test_data_generator::{params};
//use data_sample_parser::DataSampleParser;
use params::Params;
//use configs::Configs;

// This is the main function
fn main() {
	// Params object holds the arguments passed 
	let params =  Params::new();
	
	// setup logging
	log4rs::init_file(params.get_log_file(), Default::default()).unwrap();
	info!("Logging enabled...");
	
	// start up a Data Sample Parser
	//let mut dsp = DataSampleParser::new(params.get_config_file());
}