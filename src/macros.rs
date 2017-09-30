use rand::{thread_rng, Rng};

#[macro_export]
macro_rules! cum_vec_f64 {
	($a:ident) => {
		let mut cum = 0.00 as f64; 
		for v in $a.iter_mut() {
		 	cum = &cum + v.1;
		 	v.1 = &cum;
		 	println!("{}", cum);
		}
	};
}

#[macro_export]
macro_rules! random_percentage {
    ($a:ident) => {
    	use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		$a = rng.gen_range::<f64>(0 as f64,100 as f64);
    };
}