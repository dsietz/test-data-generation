use std::fs::File;
use std::io::prelude::*;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
//use serde_yaml::{Value, from_str};
use yaml_rust::YamlLoader;


pub struct DataSampleParser<'a>{
	pub issues: bool,
	opts: ArgMatches<'a>,
	cfg: String,
}

impl<'a> DataSampleParser<'a> {
	//constructor
	pub fn new() -> DataSampleParser<'a> {
		DataSampleParser{
			issues: false,
			opts: App::new("Test Data Generation")
                          .version("1.0")
                          .author("dsietz")
                          .about("Made just for you!")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Specifies the location of the Test Data Generation configuration file (default ./config/tdg.yaml)")
                               .takes_value(true)
                               .default_value("./config/tdg.yaml"))
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .value_name("FILE")
                               .help("Specifies the location of the log4rs logging configuration file (default ./config/log4rs.yaml)")
                               .takes_value(true)
                               .default_value("./config/log4rs.yaml"))     
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .possible_values(&["off","info","debug"])
                               .default_value("off")
                               .help("explain what is being done (options: off, info, debug)"))
                          .get_matches(),
           cfg: String::from(""),
		}
	}
	
	// get() functions
	pub fn get_config_file(&self) -> &str{
		&self.opts.value_of("config").unwrap_or("config/default.yaml")
	}
	
	pub fn get_log_file(&self) -> &str{
		&self.opts.value_of("log").unwrap_or("config/log4rs.yaml")
	}	
	
	pub fn get_verbose(&self) -> &str{
		&self.opts.value_of("verbose").unwrap_or("off")
	}	
		
	// unique() functions
	pub fn load_config_file(&mut self, file: &str){
		let mut f = File::open(file).expect(concat!("Error: Configuration file not found"));
		let mut contents = String::new();
		f.read_to_string(&mut self.cfg).expect("Something went wrong reading file");
		//println!("With text:\n{}", contents);
		//&*contents.clone_into(&mut self.cfg);
		//self.cfg = Some(&*contents);
		let cfg = &YamlLoader::load_from_str(&*self.cfg).expect("failed to load YAML file")[0];
		println!("{:?}", cfg);
	}
	
	pub fn runing_with_issues(&self) -> &bool{
		&self.issues
	}
	
}