extern crate test_data_generation;

#[cfg(test)]
mod tests {
	use test_data_generation::data::field::{Field, FieldType};
    
    #[test]
    // ensure a field can be set to a String value
    fn set_field(){
    	let mut fld =  Field::new("field_1".to_string(), FieldType::Text);
    	
    	fld.set("my value".to_string());
    }
    
    #[test]
    // ensure a field can retrieve a String value
    fn get_field(){
    	let mut fld =  Field::new("field_1".to_string(), FieldType::Text);
    	
    	fld.set("my value".to_string());
    	assert_eq!(fld.get().downcast_ref::<String>().unwrap().len(), 8);
    }
}