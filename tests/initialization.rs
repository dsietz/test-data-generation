#[macro_use]
extern crate test_data_generation;

use  test_data_generator::{data_sample_parser};

// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod tests {
    #[test]
    fn initialization() {
    	let dsp = data_sample_parser::DataSampleParser::new();
        assert!(dsp.status());
    }
}