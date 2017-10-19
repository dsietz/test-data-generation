#[macro_export]
macro_rules! random_percentage {
    ($a:ident) => {
    	use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		$a = rng.gen_range::<f64>(0 as f64,100 as f64);
    };
}

macro_rules! random_between {
    ($a:ident, $s:ident, $e:ident) => {
    	use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		$a = rng.gen_range::<u32>($s as u32, $e as u32);
    };
}