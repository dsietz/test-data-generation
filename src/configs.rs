//! The `configs` module provides functionality for the library to read configuration settings that the user can set in their implementation.
//!
//! # Examples
//!
//!
//! Generate some demo test data ...
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::configs::Configs;
//!
//! fn main() {
//!		// initalize a new Configs
//!		let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));
//!		cfg.load_config_file();
//!
//!		// verify the configuration file has been loaded
//!		println!("{:?}", cfg);
//! }
//! ```

//use std::path::Path;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

#[derive(Serialize, Deserialize, Debug)]
// Represents a Configs object that can be set by an implementation of the test data generation library
pub struct Configs {
    /// the file path of the test data generation library configuration file
    file: String,
}

impl Configs {
    /// Constructs a new Configs
    ///
    /// # Arguments
    ///
    /// * `path: &String - The full path name (including the file name and extension) to the configuration file.</br>
    ///
    /// #Example
    ///
    /// ```
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::configs::Configs;
    ///
    /// fn main() {
    ///		// initalize a new Configs
    ///		let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));
    ///		cfg.load_config_file();
    ///
    ///		// verify the configuration file has been loaded
    ///		println!("{:?}", cfg);
    /// }
    /// ```
    pub fn new(path: &String) -> Configs {
        let pth = path.to_string().to_owned();
        Configs { file: pth }
    }

    /// Constructs a new Configs object from a serialized (JSON) string. This is used when restoring from "archive"
    ///
    /// #Example
    ///
    /// ```
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::configs::Configs;
    ///
    /// fn main() {
    ///		let serialized = "{\"file\":\"./tests/config/tdg.yaml\"}";
    ///		let mut cfg = Configs::from_serialized(&serialized);
    ///
    ///		assert_eq!(cfg.get_config_file_path(), "./tests/config/tdg.yaml");
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> Configs {
        serde_json::from_str(&serialized).unwrap()
    }

    /// Loads the configuration file using the path that was provided during calling a new Configs object
    ///
    /// #Example
    ///
    /// ```
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::configs::Configs;
    ///
    /// fn main() {
    ///		// initalize a new Configs
    ///		let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));
    ///
    ///		// verify the configuration file path was set
    ///		println!("The configuration fiel is located at {}", cfg.get_config_file_path());
    /// }
    /// ```
    pub fn get_config_file_path(&self) -> &str {
        &self.file
    }

    /// Loads the configuration file using the path that was provided during calling a new Configs object
    ///
    /// #Example
    ///
    /// ```
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::configs::Configs;
    ///
    /// fn main() {
    ///		// initalize a new Configs
    ///		let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));
    ///		cfg.load_config_file();
    ///
    ///		// verify the configuration file has been loaded
    ///		println!("{:?}", cfg);
    /// }
    /// ```
    pub fn load_config_file(&mut self) {
        let mut f = File::open(&self.file).expect(&format!(
            "Error: Configuration file not found at {}",
            &self.file.to_string()
        ));
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading file");
        let _cfg_yaml =
            &YamlLoader::load_from_str(&*contents).expect("failed to load YAML file")[0];
        //println!("{:?}", cfg);
    }

    /// This function converts the Configs object to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```
    /// extern crate test_data_generation;
    ///
    /// use test_data_generation::configs::Configs;
    ///
    /// fn main() {
    /// 	//create a Configs object from a configuration file
    ///    	let mut cfg =  Configs::new(&String::from("./tests/config/tdg.yaml"));
    ///		cfg.load_config_file();
    ///
    ///     println!("{}", cfg.serialize());
    ///     // {"key":"r","prior_key":null,"next_key":null,"pattern_placeholder":"c","starts_with":0,"ends_with":0,"index_offset":2}
    /// }
    ///
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ensure Configs reads a valid configuration file
    fn create_config_good_cfg_file() {
        let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));

        cfg.load_config_file();
    }

    #[test]
    #[should_panic(expected = "Error: Configuration file not found at ./badpath/tdg.yaml")]
    // ensure Configs errors when reading an invalid configuration file
    fn create_config_bad_cfg_file() {
        let mut cfg = Configs::new(&String::from("./badpath/tdg.yaml"));

        cfg.load_config_file();
    }

    #[test]
    fn new_fact_from_serialized() {
        let serialized = "{\"file\":\"./tests/config/tdg.yaml\"}";
        let cfg = Configs::from_serialized(&serialized);

        assert_eq!(cfg.get_config_file_path(), "./tests/config/tdg.yaml");
    }

    #[test]
    // ensure a Configs object can be exported (to be archived) as JSON
    fn serialize() {
        let mut cfg = Configs::new(&String::from("./tests/config/tdg.yaml"));
        cfg.load_config_file();

        let serialized = cfg.serialize();
        println!("serialized : {}", serialized);

        assert_eq!(serialized, "{\"file\":\"./tests/config/tdg.yaml\"}");
    }
}
