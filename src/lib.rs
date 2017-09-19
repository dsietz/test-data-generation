#![crate_type= "lib"]
#![crate_name = "test_data_generation"]

//External
extern crate serde_yaml;
extern crate yaml_rust;
extern crate clap;
extern crate vec_map;
extern crate log;
extern crate log4rs;

//Internal
#[macro_use]
pub mod config;
pub mod test_data_generator;