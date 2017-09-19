use std::fs::File;
use std::io::prelude::*;
//use serde_yaml::{Value, from_str};
use yaml_rust::YamlLoader;
use config::params::Params;


pub struct DataSampleParser<'a>{
	pub issues: bool,
	params: Params<'a>,
	cfg: String,
}

impl<'a> DataSampleParser<'a> {
	//constructor
	pub fn new() -> DataSampleParser<'a> {
		DataSampleParser{
			issues: false,
			params: Params::new(),
            cfg: String::from(""),
		}
	}
	
	// get() functions
	pub fn get_config_file(&self) -> &str{
		&self.params.get_config_file()
	}
	
	pub fn get_log_file(&self) -> &str{
		&self.params.get_log_file()
	}	
	
	pub fn get_verbose(&self) -> &str{
		&self.params.get_verbose()
	}	
	
	// set() functions

	// unique() functions
	pub fn info(&self, msg: &str) {
		//info!(msg);
	}
		
	pub fn load_config_file(&mut self){
		let mut f = File::open(&self.params.get_config_file()).expect(concat!("Error: Configuration file not found"));
		let mut contents = String::new();
		f.read_to_string(&mut self.cfg).expect("Something went wrong reading file");
		let cfg = &YamlLoader::load_from_str(&*self.cfg).expect("failed to load YAML file")[0];
		println!("{:?}", cfg);
	}
	
	pub fn runing_with_issues(&self) -> &bool{
		&self.issues
	}
	
	pub fn warn(&self, msg: &str) {
		//warn!(msg);
	}
	
}