extern crate test_data_generation;

use test_data_generation::data_sample_parser;
use test_data_generation::profile::profile;

#[cfg(test)]
mod tests {
	use crate::data_sample_parser::DataSampleParser;
	use crate::profile::Profile;
	use std::time::{Instant};

	#[ignore]
	#[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn analyzing_word(){
    	let mut profile = Profile::new();
		let now = Instant::now();

		profile.analyze("Word");

		let d = now.elapsed().subsec_nanos();

		// should run in 1/1000 of a second (1 millisecond)
		if d > 1000000 {
			panic!("Failed: The execution time took {:?} nanoseconds.", d);
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
