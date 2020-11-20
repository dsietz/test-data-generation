extern crate test_data_generation;

use std::path::Path;
use test_data_generation::{Profile};
 
fn main() {
        // This example demonstrates the ability to conitnuously add new analyzed data to an existing profile. 
        let profile_file = "./tests/samples/demo-03";
		let mut profile = match Path::new(profile_file).exists() {
          true => {
            // use existing file
            Profile::from_file(profile_file)
          },
          false => {
            // create new file
            println!("Creating new profile: {}", profile_file);
            Profile::new_with_id("demo-03".to_string())
          },
        };

        // analyze the first data set and save the profile.
        profile.analyze("Jonny");
        profile.analyze("Jon");
        profile.analyze("Johnathon");
        profile.analyze("John");
        profile.analyze("Jonathon");
        profile.pre_generate();
        profile.save(&profile_file).unwrap();
        println!("My new name is {}", profile.generate().to_string());

        // analyze the second data set and add it to the saved profile.
        let mut profile2 = Profile::from_file(profile_file);
        profile2.analyze("Chris");
        profile2.analyze("Kris");
        profile2.analyze("Christopher");
        profile2.analyze("Christian");
        profile2.analyze("Krissy");
        profile2.pre_generate();
        profile2.save(&profile_file).unwrap();
        println!("My new name is {}", profile2.generate().to_string());

        // analyze the third data set and add it to the saved profile.
        let mut profile3 = Profile::from_file(profile_file);
        profile3.analyze("Dan");
        profile3.analyze("Danny");
        profile3.analyze("Danyl");
        profile3.analyze("Dannie");
        profile3.analyze("Danathon");
        profile3.pre_generate();
        profile3.save(&profile_file).unwrap();
        println!("My new name is {}", profile3.generate().to_string());
}