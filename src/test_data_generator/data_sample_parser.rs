use clap::App;
use clap::Arg;
use clap::ArgMatches;

pub struct DataSampleParser<'a>{
	pub issues: bool,
	opts: ArgMatches<'a>,
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
                               .help("Sets a custom config file")
                               .takes_value(true)
                               .default_value("./config/tdg.yaml"))
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .value_name("FILE")
                               .help("set a custom logging configuration file - format YAML")
                               .takes_value(true)
                               .default_value("./config/log4rs.yaml"))     
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .help("explain what is being done"))
                          .get_matches(),
		}
	}
	
	pub fn get_config_file(&self) -> &str{
		&self.opts.value_of("config").unwrap_or("config/default.yaml")
	}
	
	pub fn get_log_file(&self) -> &str{
		&self.opts.value_of("log").unwrap_or("config/log4rs.yaml")
	}	
	
	pub fn runing_with_issues(&self) -> &bool{
		&self.issues
	}
}