/*
** Data Algorithm Creator 
*/

extern crate test_data_generation;

use test_data_generation::test_data_generator::{data_sample_parser};

// This is the main function
fn main() {
	use data_sample_parser::DataSampleParser;

	let mut dsp = DataSampleParser::new();
    
    // Prints each argument on a separate line
	//for argument in env::args() {
    //	println!("{}", argument);
	//}
	
	println!( "Configuration file: {}", dsp.get_config_file() );
	
	dsp.load_config_file();

}