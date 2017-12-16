#[macro_use]
extern crate log;
extern crate test_data_generation;
extern crate log4rs;

#[cfg(test)]
mod tests {
	use log4rs;
	use test_data_generation::data_sample_parser::DataSampleParser;
	//use configs::Configs;
	
    #[test]
    #[ignore]
    // ensure that the crate (library) can be used in a executable
    fn crate_integration_test(){
		
		// setup logging
		log4rs::init_file("./config/log4rs.yaml",Default::default()).unwrap();
		info!("Logging enabled...");
		
		// start up a Data Sample Parser
		let mut dsp = DataSampleParser::new_with("./config/tdg.yaml");
		
		info!("Demo ...");
		println!("generate date:{}", dsp.demo_date());
		println!("generate person:{}", dsp.demo_person_name());
    	
    	info!("Analyzing CSV file ...");	
   		println!("reading csv file: {}", dsp.analyze_csv_file("./samples/sample-01.csv").unwrap());
   		
   		info!("Generating a first name based on the CSV file ...");	
   		println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
   		
   		println!("Generated data record: {:?}",dsp.generate_record());
		
    }  
}