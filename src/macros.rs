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
