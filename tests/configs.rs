extern crate test_data_generation;

use test_data_generation::configs;

#[cfg(test)]
mod tests {
	use configs::Configs;
    
    #[test]
    #[ignore]
    // ensure Configs reads a valid configuration file
    fn create_config_good_cfg_file(){
    	let mut cfg =  Configs::new("./tests/config/tdg.yaml");
    	
    	cfg.load_config_file();
    }
    
    #[test]
    #[ignore]    
    // ensure Configs errors when reading an invalid configuration file
    fn create_config_bad_cfg_file(){
    	let mut cfg =  Configs::new("./badpath/tdg.yaml");
    	
    	cfg.load_config_file();
    }
    
    #[test]
    fn new_fact_from_serialized(){
    	let serialized = "{\"file\":\"./tests/config/tdg.yaml\"}";
    	let cfg = Configs::from_serialized(&serialized);
    	
    	assert_eq!(cfg.get_config_file_path(), "./tests/config/tdg.yaml");
    }
    
    #[test]
    // ensure a Configs object can be exported (to be archived) as JSON
    fn serialize(){
        let mut cfg =  Configs::new("./tests/config/tdg.yaml");
        cfg.load_config_file();
        
    	let serialized = cfg.serialize();
    	println!("serialized : {}",serialized);
    	
		assert_eq!(serialized,"{\"file\":\"./tests/config/tdg.yaml\"}");
    }    
}