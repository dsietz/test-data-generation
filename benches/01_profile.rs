#![feature(test)]

extern crate test_data_generation;
extern crate test;

use test_data_generation::profile;
use profile::profile::Profile;
use test::Bencher;

#[bench]
fn analyze_word(b: &mut Bencher) {
	let mut profil =  Profile::new();

    b.iter(|profil| profil.analyze("Word"));
}
