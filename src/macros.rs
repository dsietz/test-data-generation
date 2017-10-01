/**

macro_rules! cum_sizerankmap {
	($a:ident) => {	
		let x = u32::min_value();
		let mut iter = $a.iter().scan((&x, 0.00 as &f64), |state, &(k, v)| { 
			*state = (k, state.1 + v);
			Some(*state)
		}).collect::<Vec<(_,_)>>();
		$a = iter;
	};
}


macro_rules! cum_sizerankmap {
	($a:ident) => {
		let mut iter = $a.iter().scan((0 as u32, 0.00 as f64), |state, &(k, v)| {
			*state = (*k, state.1 + v);
			Some(*state)
		}).collect::<Vec<(_,_)>>();
		//$a = iter;
	};
}
**/

#[macro_export]
macro_rules! random_percentage {
    ($a:ident) => {
    	use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		$a = rng.gen_range::<f64>(0 as f64,100 as f64);
    };
}