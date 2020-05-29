extern crate test_data_generation;

use test_data_generation::data_sample_parser::DataSampleParser;
 
fn main() {

		// This example demonstrates the basic feature of the library to generate dates and people's names
		// using sample data that is included in the library. 
		
		// initalize a new DataSampelParser
        let mut dsp = DataSampleParser::new();
        
        dsp.analyze_csv_file(&String::from("./tests/samples/sample-names.csv")).unwrap();

        // generate some test data using the demo functions
        println!("My new name is {} {}", dsp.generate_record()[0], dsp.generate_record()[1]);
}