extern crate test_data_generation;

use test_data_generation::test_data_generator::{data_sample_parser};

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
    	let dsp = DataSampleParser::new();
        assert!(!dsp.runing_with_issues());
    }
    
    #[test]
    //the data sample parser configuration parameters are ready
    fn load_configuration() {
    	let mut dsp = DataSampleParser::new();
    	dsp.load_config_file("./src/bin/config/tdg.yaml");
        assert!(!dsp.get_config_file().is_empty());
    }
    
    
    #[test]
    //the data sample parser configuration parameters are ready
    fn configuration() {
    	let dsp = DataSampleParser::new();
        assert!(!dsp.get_config_file().is_empty());
    }
    
    #[test]
    //the data sample parser logging parameters are ready
    fn logging() {
    	let dsp = DataSampleParser::new();
        assert!(!dsp.get_log_file().is_empty());
    }
}