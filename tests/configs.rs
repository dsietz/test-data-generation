extern crate test_data_generation;

use test_data_generation::configs;

#[cfg(test)]
mod tests {
	use configs::Configs;
    
    #[test]
    // ensure Configs reads a valid configuration file
    fn create_config_good_cfg_file(){
    	let mut cfg =  Configs::new("./target/debug/config/tdg.yaml");
    	
    	cfg.load_config_file();
    }
    
    #[test]
    #[ignore]    
    // ensure Configs errors when reading an invalid configuration file
    fn create_config_bad_cfg_file(){
    	let mut cfg =  Configs::new("./badpath/tdg.yaml");
    	
    	cfg.load_config_file();
    }
}