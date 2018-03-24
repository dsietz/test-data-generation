extern crate test_data_generation;

#[cfg(test)]
mod tests {
	use test_data_generation::data_sample_parser::DataSampleParser;
	use std::time::{SystemTime};
	
    #[test]
    
    // ensure that the crate (library) can be used in a executable
    fn analyze_performance_test(){
		// start up a Data Sample Parser
		let mut dsp = DataSampleParser::new();
		
		let start = SystemTime::now();
    
    	// 15K rows with 2 fields
   		println!("reading csv file: {}", dsp.analyze_csv_file("./tests/samples/sample-names-1k.csv").unwrap());		
   		
   		let finish = SystemTime::now();
   		
   		//println!("started at {:?}", finish.duration_since(start));
   		// started at Ok(Duration { secs: 259, nanos: 522843800 })
   		assert!(finish.duration_since(start).unwrap().as_secs() < 60);
    }  
}