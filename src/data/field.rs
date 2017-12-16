// https://doc.rust-lang.org/beta/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
// https://stackoverflow.com/questions/25740916/how-do-you-actually-use-dynamically-sized-types-in-rust
// https://stackoverflow.com/questions/24857831/is-there-any-downside-to-overloading-functions-in-rust-using-a-trait-generic-f

use std::any::{Any, TypeId};

/// this trait is used to overload the set() function
pub trait ConvertToBytes {
	fn convert_to_bytes<'a>(self) -> Vec<u8>;
}

/// implement the trait for the integer data type
impl ConvertToBytes for u8 {
	fn convert_to_bytes<'a>(self) -> Vec<u8> {
		return vec![self]
	}
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
	pub data_type: Option<TypeId> ,
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
	/// 	let mut fld = Field::new("field_1".to_string());
	///		
	///		fld.set("John".to_string());
	///		println!("The value is {}",fld.get().downcast_ref::<String>().unwrap());
	/// }
	/// ```
	pub fn new(field_name: String) -> Field {
		Field{
			name: field_name,
			data_type: None,
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
	/// 	let mut fld = Field::new("first name".to_string());
	///     fld.set("John".to_string());
	/// }
	/// ```
	pub fn set<T:ConvertToBytes + 'static>(&mut self, val: T) {
		//set the field value
		self.value = val.convert_to_bytes();

		//set the field type
		self.data_type = Some(TypeId::of::<T>());
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
	/// 	let mut fld = Field::new("field_1".to_string());
	///		
	///		fld.set("John".to_string());
	///		println!("The value is {}",fld.get().downcast_ref::<String>().unwrap());
	/// }
	/// ```
	pub fn get(&self) -> Box<Any> {
		Box::new(String::from_utf8(self.value.to_vec()).unwrap())
	}

	/// This function gets the data type of the Field based on the value. 
	/// It is used in combination with the get() function
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::data::field::Field;
	/// use std::any::TypeId;
	///	
	/// fn main() {
	/// 	let mut fld = Field::new("first name".to_string());
	///     let n1 :u8 = 100;
	///  
	///		fld.set(n1);
	///     assert_eq!(fld.get_field_type(),TypeId::of::<u8>());
	///     //println!("The field value is {}", fld.get().downcast_ref::<T:fld.get_field_type()>().unwrap());
	/// }
	/// ```
	pub fn get_field_type(&mut self) -> TypeId {
		self.data_type.unwrap()
	}
}