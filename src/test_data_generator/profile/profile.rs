use test_data_generator::profile::pattern::{Pattern};

pub struct Profile{
	pub patterns: Vec<String>,
}

impl Profile {
	//constructor
	pub fn new() -> Profile {
		Profile{
			patterns: Vec::new(),
		}
	}
	
	// methods
	pub fn analyze(&mut self, entity: &str) {
		let mut pattrn =  Pattern::new();
    	self.patterns.push(pattrn.analyze(entity).to_string()); 
	} 
	
	
	pub fn map_reduce(&self) {
		/*************************************************************************
     	* "Map" phase
     	*
     	* Divide our data into segments, and apply initial processing
     	************************************************************************/
     
		// Iterate over the data segments.
    	// .enumerate() adds the current loop index to whatever is iterated
    	// the resulting tuple "(index, element)" is then immediately
    	// "destructured" into two variables, "i" and "data_segment" with a
    	// "destructuring assignment"
		for (i,data_segment) in self.patterns.iter().enumerate() {
			println!("{},{}",i,data_segment);
			
			// Process each data segment in a separate thread
        	//
        	// spawn() returns a handle to the new thread,
        	// which we MUST keep to access the returned value
        	//
       		// 'move || -> u32' is syntax for a closure that:
        	// * takes no arguments ('||')
        	// * takes ownership of its captured variables ('move') and
        	// * returns an unsigned 32-bit integer ('-> u32')
        	//
        	// Rust is smart enough to infer the '-> u32' from 
        	// the closure itself so we could have left that out.
        	//
        	// TODO: try removing the 'move' and see what happens
        	
		}
	}
	
}