extern crate test_data_generation;

use test_data_generation::data_sample_parser;

#[cfg(test)]
mod tests {
	use data_sample_parser::DataSampleParser;
	use std::fs::File;
	use std::io::prelude::*;
	use std::io::BufReader;

    #[test]
    // ensure the Data Sample Parser can be restored from archived file
    fn from_file(){
    	let mut dsp = DataSampleParser::from_file(&String::from("./tests/samples/sample-00-dsp"));
    	println!("Sample data is [{:?}]", dsp.generate_record()[0]);

    	assert_eq!(dsp.generate_record()[0], "OK".to_string());
    }

	#[test]
	// ensure the Data Sample Parser can read all the headers from teh csv file
	fn read_headers(){
		let mut dsp = DataSampleParser::new();

	    dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
	    let headers = dsp.extract_headers();
	   
	    assert_eq!(headers.len(), 2);
	}

    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file(){
    	let mut dsp =  DataSampleParser::new();

    	assert_eq!(dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap(), 1);
    }

	#[test]
	// ensure DataSampleParser can analyze a csv formatted text
	fn parse_csv_data(){
		let mut dsp =  DataSampleParser::new();
		let mut data = String::from("");
		data.push_str("\"firstname\",\"lastname\"\n");
		data.push_str("\"Aaron\",\"Aaberg\"\n");
		data.push_str("\"Aaron\",\"Aaby\"\n");
		data.push_str("\"Abbey\",\"Aadland\"\n");
		data.push_str("\"Abbie\",\"Aagaard\"\n");
		data.push_str("\"Abby\",\"Aakre\"");

		assert_eq!(dsp.analyze_csv_data(&data).unwrap(), 1);
	}

    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn generate_field_from_csv_file(){
    	let mut dsp =  DataSampleParser::new();

    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    	println!("Generated data for first name {}",dsp.generate_by_field_name("firstname".to_string()));
    }

    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn generate_record_from_csv_file(){
    	let mut dsp =  DataSampleParser::new();

    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    	assert_eq!(dsp.generate_record().len(), 2);
    }

    #[test]
    // ensure DataSampleParser can analyze a csv formatted file
    fn parse_csv_file_bad(){
    	let mut dsp =  DataSampleParser::new();

    	assert_eq!(dsp.analyze_csv_file(&String::from("./badpath/sample-01.csv")).is_err(), true);
    }

    #[test]
    // ensure the DataSampleParser object can be saved to file
    fn save(){
    	let mut dsp =  DataSampleParser::new();
    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-00.csv")).unwrap();

    	assert_eq!(dsp.save(&String::from("./tests/samples/sample-00-dsp")).unwrap(), true);
    }

    #[test]
    // ensure the DataSampleParser object can recognize the difference between realistic data and unrealistic generated data
    fn levenshtein_test(){
    	let mut dsp =  DataSampleParser::new();

    	assert_eq!(dsp.levenshtein_distance(&"kitten".to_string(), &"sitting".to_string()), 3 as usize);
    }

	#[test]
	// ensure the DataSampleParser object can recognize the difference between realistic data and unrealistic generated data
	fn realistic_data_test(){
		let mut dsp =  DataSampleParser::new();

		assert_eq!(dsp.realistic_test(&"kitten".to_string(), &"sitting".to_string()), 76.92307692307692 as f64);
	}

    #[test]
    // demo test
    fn demo(){
    	let mut dsp = DataSampleParser::new();
    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();

    	println!("My new name is {} {}", dsp.generate_record()[0], dsp.generate_record()[1]);

    	assert!(true);
    }

    #[test]
    // ensure the DataSampleParser object can generate test data as a csv file
    fn extract_headers_from_sample(){
    	let mut dsp =  DataSampleParser::new();

    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    	let headers = dsp.extract_headers();

    	assert_eq!(headers.len(), 2);
    }

    #[test]
    // ensure the DataSampleParser object can generate test data as a csv file
    fn generate_csv_test_data_from_sample(){
    	let mut dsp =  DataSampleParser::new();

    	dsp.analyze_csv_file(&String::from("./tests/samples/sample-01.csv")).unwrap();
    	dsp.generate_csv(100, &String::from("./tests/samples/generated-01.csv")).unwrap();

		let generated_row_count = match File::open(format!("{}","./tests/samples/generated-01.csv")) {
			Err(_e) => {
				0
			},
			Ok(f) => {
				let mut count = 0;
				let bf = BufReader::new(f);

				for _line in bf.lines() {
					count += 1;
				}

				count
			},
		};

    	assert_eq!(generated_row_count, 101);
    }
}
