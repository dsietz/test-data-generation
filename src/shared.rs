use std::mem;

/// This function converts a String to a &'static str</br>
///
/// # Arguments
///
/// * `s: String` - The textual string to be converted.</br>
///
/// # Example
///
/// ```
/// extern crate test_data_generation;
///
/// use test_data_generation::shared;
///
/// fn main() {
///     let my_string = String::from("Hello World");
///		let static_str =  shared::string_to_static_str(my_string);
/// }
/// ```
pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
