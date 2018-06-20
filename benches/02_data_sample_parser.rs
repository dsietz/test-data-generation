#![feature(test)]

extern crate test_data_generation;
extern crate test;

use test_data_generation::data_sample_parser::DataSampleParser;
use test::Bencher;

#[bench]
fn analyze_csv_file(b: &mut Bencher) {
	// start up a Data Sample Parser
	let mut dsp = DataSampleParser::new();

    // 15K rows with 2 fields
    b.iter(|dsp| dsp.analyze_csv_file("./tests/samples/sample-names-1k.csv").unwrap());
}
