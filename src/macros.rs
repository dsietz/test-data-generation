/// This macro calculates the levenshtein distance between 2 strings.
/// See: https://crates.io/crates/levenshtein
///
/// # Arguments
///
/// * `control: &String` - The string to compare against. This would be the real data from the data sample.</br>
/// * `experiment: &String` - The string to compare. This would be the generated data for which you want to find the distance.</br>
///
/// #Example
///
/// ```
/// # #[macro_use] extern crate test_data_generation; extern crate levenshtein;
/// # fn main() {
///   // let kitten = String::from("kitten");
///   // let sitting = String::from("sitting");
///   // assert_eq!(levenshtein_distance!(&kitten, &sitting), 3 as usize);
/// # }
///
#[macro_export]
macro_rules! levenshtein_distance {
    ( $c:ident, $e:ident ) => {
        {
            use levenshtein;

            levenshtein::levenshtein($c, $e)
        }
    }
}

/// This macro generates a random number between 0 and 100.
/// Returns a f64.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate test_data_generation; extern crate rand;
/// # fn main() {
/// 	let rnd: f64 = random_percentage!();
///     println!("Your random number is {}", rnd);
/// # }
/// ```
#[macro_export]
macro_rules! random_percentage {
    ( $( $x:expr ),* ) => {
        {
    	      use rand::{thread_rng, Rng};

		      let mut rng = thread_rng();

		      rng.gen_range::<f64>(0 as f64,100 as f64)

        }
    };
}

/// This macro generates a random number for a given range.
/// Returns a u32.
///
/// # Arguments
///
/// * `a: u32` - The lowest number of the range to use for the random number.</br>
/// * `b: u32` - The highest number of the range to use for the random number.</br>
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate test_data_generation; extern crate rand;
/// # fn main() {
/// 	//let rnd: u32 = random_between!(0, 100);
///     //println!("Your random number is {}", rnd);
/// # }
/// ```
#[macro_export]
macro_rules! random_between {
    ($a:ident, $b:ident) => {
        {
    	    use rand::{thread_rng, Rng};

		    let mut rng = thread_rng();
	        let nbr = rng.gen_range::<u32>($a as u32, $b as u32);

            nbr
        }
    };
}

/// This function calculates the percent difference between 2 strings.
///
/// # Arguments
///
/// * `control: &String` - The string to compare against. This would be the real data from the data sample.</br>
/// * `experiment: &String` - The string to compare. This would be the generated data for which you want to find the percent difference.</br>
///
/// #Example
///
/// ```
/// # #[macro_use] extern crate test_data_generation; extern crate levenshtein;
/// # fn main() {
///     // let kitten = String::from("kitten");
///     // let sitting = String::from("sitting");
///		// assert_eq!(realistic_test!(&kitten, &sitting), 76.92307692307692 as f64);
/// # }
///
#[macro_export]
macro_rules! realistic_test {
    ( $c:ident, $e:ident ) => {
        {
            let ld: f64 = levenshtein_distance!($c, $e) as f64;
    		let total: f64 = $c.len() as f64 + $e.len() as f64;
    		let diff: f64 = total - ld;
    		(1 as f64 - ((total - diff)/total)) * 100   as f64
        }
    }
}
