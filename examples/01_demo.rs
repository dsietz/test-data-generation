extern crate test_data_generation;

use test_data_generation::data_sample_parser::DataSampleParser;
 
fn main() {

		// This example demonstrates the basic feature of the library to generate dates and people's names
		// using demo sample that is included in the library. 
		
		// initalize a new DataSampelParser
		let dsp = DataSampleParser::new();

		// generate some test data using the demo functions
		println!("generate date:{}", dsp.demo_date());
		println!("generate person:{}", dsp.demo_person_name());
}