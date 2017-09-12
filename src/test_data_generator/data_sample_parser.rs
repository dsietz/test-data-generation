pub struct DataSampleParser{
	pub issues: bool,
}

impl DataSampleParser {
	//constructor
	pub fn new() -> DataSampleParser {
		DataSampleParser{
			issues: false,
		}
	}	
	
	pub fn runing_with_issues(&self) -> &bool{
		&self.issues
	}
}