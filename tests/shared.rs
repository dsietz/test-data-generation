extern crate test_data_generation;

#[cfg(test)]
mod tests {
    use test_data_generation::shared;

	#[test]
    // ensure the conversion of String to &'static str
    fn to_static_str(){
        let static_str: &'static str = "Hello World";
        let my_string = String::from("Hello World");
        let my_static_str =  shared::string_to_static_str(my_string);

        assert_eq!(static_str, my_static_str);
    }
}
