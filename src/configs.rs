//! Configs
//! 
//! 
//! 
//! 
//! 
//! 

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
//use test_data_generator::params::Params;

pub struct Configs{
	file: &'static str,
}

impl Configs {
	pub fn new(path: &'static str) -> Configs {
		Configs{
			file: path,
		}
	}
	
	// get() methods
	pub fn get_config_file(&self) -> &str{
		&self.file
	}
	
	// set() methods
	
	// unique methods
	pub fn load_config_file(&mut self, ){
		let mut f = File::open(&self.file).expect(concat!("Error: Configuration file not found"));
		let mut contents = String::new();
		f.read_to_string(&mut contents).expect("Something went wrong reading file");
		let cfg_yaml = &YamlLoader::load_from_str(&*contents).expect("failed to load YAML file")[0];
		//println!("{:?}", cfg);
	}
}