use test_data_generator::configs::Configs;

pub struct DataSampleParser{
	pub issues: bool,
	cfg: Configs,
}

impl DataSampleParser {
	//constructor
	pub fn new(path: &'static str) -> DataSampleParser {
		DataSampleParser{
			issues: false,
            cfg: Configs::new(path),
		}
	}
	
	// get() functions
		
	// set() functions

	// unique() functions	
	pub fn runing_with_issues(&self) -> &bool{
		&self.issues
	}	
}