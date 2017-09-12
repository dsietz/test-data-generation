#[macro_use]
extern crate test_data_generation;

use test_data_generation::test_data_generator::{data_sample_parser};
//use data_sample_parser::DataSampleParser;

// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {

	use data_sample_parser::DataSampleParser;
    
    #[test]
    //the data sample parser starts up without issues
    fn initialization() {
    	let mut dsp = DataSampleParser::new();
        assert!(!dsp.runing_with_issues());
    }
}