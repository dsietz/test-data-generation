#![crate_type= "lib"]
#![crate_name = "test_data_generation"]

//External
extern crate serde_yaml;
extern crate yaml_rust;
extern crate clap;
extern crate log;
extern crate regex;
extern crate rand;

#[macro_use]
pub mod macros;

//Internal
pub mod test_data_generator;