extern crate test_data_generation;

#[cfg(test)]
mod tests {
    use test_data_generation::engine::{Engine};

    struct Xtest{}
    impl Engine for Xtest{}

    #[test]
    fn test_pattern_definition_analyze_multithread(){
        let words = vec!("word-one","word-two","word-three","word-four","word-five");

        let results = Xtest::analyze_entities(words);

        println!("{:?}", results);
        assert_eq!(results.len(), 5);
    } 
}