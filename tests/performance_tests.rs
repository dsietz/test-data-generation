extern crate test_data_generation;

use test_data_generation::data_sample_parser;

#[cfg(test)]
mod tests {
	use crate::data_sample_parser::DataSampleParser;
	use test_data_generation::engine::{Engine};
	use std::time::{Instant};

	struct Xtest{}
    impl Engine for Xtest{}

	#[ignore]
	#[test]
    // Performance Test 
    fn analyzing_word(){
		let now = Instant::now();
		let words = vec!("word-one".to_string(),"word-two".to_string(),"word-three".to_string(),"word-four".to_string(),"word-five".to_string());

        let _results = Xtest::analyze_entities(words);

		let d = now.elapsed().subsec_millis();

		// should run in less than 10 millisecond
		if d > 10 {
			panic!("Failed: The execution time took {:?} milliseconds.", d);
        }
    }

	#[ignore]
	#[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn analyzing_csv_file_15k(){
    	let mut dsp =  DataSampleParser::new();
		let now = Instant::now();

		dsp.analyze_csv_file(&String::from("./tests/samples/sample-names.csv")).unwrap();

		if now.elapsed().as_secs() > 60 {
			panic!("Failed: The execution time took {:?} seconds.", now.elapsed().as_secs());
		}
    }
}
