extern crate test_data_generation;

use test_data_generation::data_sample_parser;

#[cfg(test)]
mod tests {
	use data_sample_parser::DataSampleParser;
	//use std::fs::File;
	//use std::io::prelude::*;
    
    #[test]
    // ensure the Data Sample Parser can be restored from archived file
    fn from_file(){
    	let mut dsp = DataSampleParser::from_file("./tests/samples/sample-00-dsp");
    	println!("Sample data is [{:?}]", dsp.generate_record()[0]);
    	
    	assert_eq!(dsp.generate_record()[0], "OK".to_string());
    }
    
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
    fn generate_record_from_csv_file(){
    	let mut dsp =  DataSampleParser::new();
    	
    	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
    	assert_eq!(dsp.generate_record().len(), 2);
    }    
    
    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file_bad(){
    	let mut dsp =  DataSampleParser::new();
    	
    	assert_eq!(dsp.analyze_csv_file("./badpath/sample-01.csv").is_err(), true);
    }
    
    #[test]
    // ensure the DataSampleParser object can be saved to file
    fn save(){
    	let mut dsp =  DataSampleParser::new();  
    	dsp.analyze_csv_file("./tests/samples/sample-00.csv").unwrap();
    	
    	assert_eq!(dsp.save("./tests/samples/sample-00-dsp").unwrap(), true);
    }    
}