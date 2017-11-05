// https://doc.rust-lang.org/beta/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
// https://stackoverflow.com/questions/25740916/how-do-you-actually-use-dynamically-sized-types-in-rust
// https://stackoverflow.com/questions/24857831/is-there-any-downside-to-overloading-functions-in-rust-using-a-trait-generic-f

use std::any::Any;

/// this trait is used to overload the set() function
pub trait ConvertToBytes {
	fn convert_to_bytes<'a>(self) -> Vec<u8>;
}

/// implement the trait for the String data type
impl ConvertToBytes for String {
	fn convert_to_bytes<'a>(self) -> Vec<u8> {
		return self.as_bytes().to_vec()
	}
}

/// This enum is used to specify the possible data types a Field can represent
pub enum FieldType {
	/// any textual data type (String, char)
	Text,
	/// any non-decimal numeric data type (int, usize, u8, u16, u64)
	Number,	
}

/// Represents a Fact for a character in a sample data entity that has been analyzed
pub struct Field {
	/// the textual name of the field
	pub name: String,
	/// the data type of the field value
	data_type: u8,
	/// the value of the field
	pub	value: Vec<u8>,
}

impl Field {
	/// Constructs a new Field
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data::field::{Field, FieldType};
	///	
	/// fn main() {
	/// 	let mut fld = Field::new("field_1".to_string(), FieldType::Text);
	///		
	///		fld.set("John".to_string());
	///		println!("The value is {}",fld.get().downcast_ref::<String>().unwrap());
	/// }
	/// ```
	pub fn new(name: String, field_type: FieldType) -> Field {
		Field{
			name: String::new(),
			data_type: match field_type {
				FieldType::Text => 1 as u8,
				FieldType::Number => 2 as u8
			},
			value: Vec::new(),
		}
	}
	
	/// This function sets the value of the Field. 
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data::field::{Field, FieldType};
	///	
	/// fn main() {
	/// 	let mut fld = Field::new("first name".to_string(), FieldType::Text);
	///     fld.set("John".to_string());
	/// }
	/// ```
	pub fn set<T:ConvertToBytes>(&mut self, val:T) {
		//println!("data type is {}",typeof(val));
		self.value = val.convert_to_bytes();
	}
	
	/// This function returns the value for the Field
	/// Since a Box is return from this function, the actual value can be downcast to the proper type
	/// example: _.get().downcast_ref::<String>().unwrap()_
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data::field::{Field, FieldType};
	///	
	/// fn main() {
	/// 	let mut fld = Field::new("field_1".to_string(), FieldType::Text);
	///		
	///		fld.set("John".to_string());
	///		println!("The value is {}",fld.get().downcast_ref::<String>().unwrap());
	/// }
	/// ```
	pub fn get(&self) -> Box<Any> {
		Box::new(String::from_utf8(self.value.to_vec()).unwrap())
	}
}