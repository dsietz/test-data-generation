#![crate_type= "lib"]
#![crate_name = "test_data_generation"]

//External
#[macro_use]
extern crate serde_yaml;
extern crate yaml_rust;
extern crate clap;
extern crate log;

//Internal
pub mod test_data_generator;