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
    	
    	assert!(dsp.analyze_csv_file("./tests/samples/sample-01.csv"));
    }
    
    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file_bad(){
    	let mut dsp =  DataSampleParser::new();
    	
    	assert_eq!(dsp.analyze_csv_file("./badpath/sample-01.csv"), false);
    }
}