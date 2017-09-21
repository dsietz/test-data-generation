use clap::{Arg, App, ArgMatches};

pub struct Params<'a>{
	opts: ArgMatches<'a>,
}

impl<'a> Params<'a> {
	//constructor
	pub fn new() -> Params<'a> {
		Params{
			opts: App::new("Test Data Generation")
                          .version("1.0")
                          .author("dsietz")
                          .about("Made just for you!")
                          .arg(Arg::with_name("tool")
                               .short("t")
                               .long("tool")
                               .possible_values(&["data-sampler","data-generator"])
                               .required(true)
                               .default_value("data-sampler")
                               .help("define the tool to use (options: data-sampler, data-generator)"))
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Specifies the location of the Test Data Generation configuration file (default tdg.yaml)")
                               .takes_value(true)
                               .default_value("tdg.yaml"))
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .value_name("FILE")
                               .help("Specifies the location of the log4rs logging configuration file (default log4rs.yaml)")
                               .takes_value(true)
                               .default_value("log4rs.yaml"))     
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .possible_values(&["off","info","debug"])
                               .default_value("off")
                               .help("explain what is being done (options: off, info, debug)"))
                          .get_matches(),
		}
	}
	
	// get() functions
	pub fn get_config_file(&self) -> &str{
		&self.opts.value_of("config").unwrap()
	}
	
	pub fn get_log_file(&self) -> &str{
		&self.opts.value_of("log").unwrap()
	}
	
	pub fn get_tool(&self) -> &str{
		&self.opts.value_of("tool").unwrap()
	}
	
	pub fn get_verbose(&self) -> &str{
		&self.opts.value_of("verbose").unwrap()
	}
	
	//set() functions
}