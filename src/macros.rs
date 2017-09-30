use rand::{thread_rng, Rng};

#[macro_export]
macro_rules! random_percentage {
    ($a:ident) => {
    	use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		$a = rng.gen_range::<f64>(0 as f64,100 as f64);
    };
}