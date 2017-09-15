/*
** Test Data Generation 
*/

extern crate test_data_generation;
//implement log4rs

use test_data_generation::test_data_generator::{data_sample_parser};

// This is the main function
fn main() {
	use data_sample_parser::DataSampleParser;

	let dsp = DataSampleParser::new();
    
    // Print text to the console
    println!("Hello World!");
}