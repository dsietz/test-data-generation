extern crate test_data_generation;

use test_data_generation::data_sample_parser;

#[cfg(test)]
mod tests {
	use data_sample_parser::DataSampleParser;
	//use std::fs::File;
	//use std::io::prelude::*;
    
    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file(){
    	let mut dsp =  DataSampleParser::new();
    	
    	assert_eq!(dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap(), 1);
    }
    
    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn generate_field_from_csv_file(){
    	let mut dsp =  DataSampleParser::new();
    	
    	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
    	println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
    }
    
    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file_bad(){
    	let mut dsp =  DataSampleParser::new();
    	
    	assert_eq!(dsp.analyze_csv_file("./badpath/sample-01.csv").is_err(), true);
    }
}