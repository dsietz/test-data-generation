use test_data_generator::profile::pattern::{Pattern};
use test_data_generator::profile::fact::{Fact};
use std::collections::BTreeMap;
use std::ops::AddAssign;
//use std::thread;
use crossbeam;

type PatternMap = BTreeMap<String, u32>;
//type PatternRankMap  = BTreeMap<&str, f64>;
type SizeMap = BTreeMap<u32, u32>;
type SizeRankMap  = BTreeMap<u32, f64>;

pub struct Profile {
	pub patterns: PatternMap,
	pub pattern_total: u32,
	pub pattern_keys: Vec<String>,
	pub pattern_vals: Vec<u32>,
	pub pattern_percentages: Vec<(String, f64)>,
	pub pattern_ranks: Vec<(String, f64)>,
	pub sizes: SizeMap,
	pub size_total: u32,
	pub size_ranks: Vec<(u32, f64)>,
	pub processors: u8,
	pub facts: Vec<Vec<Fact>>,
}

impl Profile {
	//constructor
	pub fn new() -> Profile {
		Profile {
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_keys: Vec::new(),
			pattern_vals: Vec::new(),
			pattern_percentages: Vec::new(),
			pattern_ranks: Vec::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: Vec::new(), 
			processors: 4,
			facts: Profile::new_facts(4),
		}
	}
	
	pub fn new_with(p: u8) -> Profile {
		Profile {
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_keys: Vec::new(),
			pattern_vals: Vec::new(),
			pattern_percentages: Vec::new(),
			pattern_ranks: Vec::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: Vec::new(), 
			processors: p,
			facts: Profile::new_facts(p),
		}
	}

	// methods
	pub fn analyze(&mut self, entity: &str) {
		let mut pattrn =  Pattern::new();
		
		// analyze patterns
		let rslt = pattrn.analyze(entity);
		
		// balance the storing of facts across all the vectors that can be processed in parallel
		let mut i = 0;
		for f in rslt.1.into_iter() {			
			if i == self.processors {
				i = 0;
			}

			self.facts[i as usize].push(f);
			i = i + 1;
			
		}
		
		// store the pattern
		AddAssign::add_assign(self.patterns.entry(rslt.0.to_string()).or_insert(0), 1);
		
		// store the total number of patterns generated so far
		self.pattern_total = self.patterns.values().sum::<u32>();
		
		// analyze sizes
		AddAssign::add_assign(self.sizes.entry(pattrn.size).or_insert(0), 1);
		self.size_total = self.sizes.values().sum::<u32>();
		
		self.pattern_keys = self.patterns.keys().cloned().collect();
		self.pattern_vals = self.patterns.values().cloned().collect();
	} 
	
	// Reference: https://users.rust-lang.org/t/cannot-infer-an-appropriate-lifetime-for-autoref/13360/3
	pub fn cum_patternmap(&mut self) {
		// calculate the percentage by patterns
		// -> {"CcvccpSCvcc": 14.285714285714285, "CvccvccpSCvccvc": 14.285714285714285, "CvccvccpSCvccvv": 28.57142857142857, "CvcvcccpSCcvcv": 14.285714285714285, "CvcvpSCvccc": 14.285714285714285, "V~CcvvcpSCvccc": 14.285714285714285}	
		let n = self.patterns.len();
		
		for m in 0..n {
			self.pattern_percentages.push((self.pattern_keys[m].clone(), (self.pattern_vals[m] as f64 / self.pattern_total as f64) * 100.0));
		}

		// sort the ranks by percentages in decreasing order
		// -> [("CvccvccpSCvccvv", 28.57142857142857), ("CcvccpSCvcc", 14.285714285714285), ("CvccvccpSCvccvc", 14.285714285714285), ("CvcvcccpSCcvcv", 14.285714285714285), ("CvcvpSCvccc", 14.285714285714285), ("V~CcvvcpSCvccc", 14.285714285714285)]
		self.pattern_percentages.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());

		// calculate the cumulative sum of the pattern rankings
		// -> [("CvccvccpSCvccvv", 28.57142857142857), ("CcvccpSCvcc", 42.857142857142854), ("CvccvccpSCvccvc", 57.14285714285714), ("CvcvcccpSCcvcv", 71.42857142857142), ("CvcvpSCvccc", 85.7142857142857), ("V~CcvvcpSCvccc", 99.99999999999997)] 
		let mut rank: f64 = 0.00;
		
		for pttrn in self.pattern_percentages.iter() {
			let tmp = pttrn.1 + rank;
			self.pattern_ranks.push((pttrn.0.clone(),tmp));
			rank = tmp;
		}
	}
	
	/// calculates the sizes to use by the chance they will occur (as cumulative percentage) in decreasing order
	pub fn cum_sizemap(&mut self) {
		// calculate the percentage by sizes
		// -> {11: 28.57142857142857, 14: 14.285714285714285, 15: 57.14285714285714}
		let mut size_ranks = SizeRankMap::new();
		
		for key in self.sizes.keys(){
			size_ranks.insert(*key, (*self.sizes.get(key).unwrap() as f64 / self.size_total as f64)*100.0);
		}
	
		// sort the ranks by percentages in decreasing order
		// -> [(15, 57.14285714285714), (11, 28.57142857142857), (14, 14.285714285714285)]
		let mut sizes = size_ranks.iter().collect::<Vec<_>>();
		sizes.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
		
		// calculate the cumulative sum of the size rankings
		// -> [(15, 57.14285714285714), (11, 85.71428571428571), (14, 100)]
		self.size_ranks = sizes.iter().scan((0 as u32, 0.00 as f64), |state, &(&k, &v)| {
			*state = (k, state.1 + &v);
			Some(*state)
		}).collect::<Vec<(_,_)>>();	
	}

	pub fn pre_generate(&mut self){
		self.cum_sizemap();
		self.cum_patternmap();
	}
	
	pub fn generate(&mut self) -> bool{
		// first, determine the length of the entity
		 // 1. get a random number
	 	let mut s: f64 = 0 as f64;
	 	random_percentage!(s);
	 	 // 2. find the first size that falls within the percentage chance of occurring
		let size = self.size_ranks.iter().find(|&&x|&x.1 >= &s).unwrap().0;	 	
		
		// second, determine the pattern to use
		let pattern = self.pattern_ranks.iter().find(|x|&x.1 >= &s && x.0.len() == size as usize).unwrap().clone();		
		
		// build the entity using facts that adhere to the pattern 
		//let fact = self.find_facts(pattern.0);
		
		
		if size > 0 { true } else { false }
	}
	
	fn new_facts(p: u8) -> Vec<Vec<Fact>> {
		let mut vec_main = Vec::new();
		
		for _ in 0..p {  
			vec_main.push(Vec::new());
		}

		vec_main
	}


	pub fn find_facts(&self, pattern: String) -> Vec<Fact> {
		let pattern_chars = pattern.chars().collect::<Vec<char>>();
		
		for (idx, ch) in pattern_chars.iter().enumerate() {
			//println!("pattern_chars index: {:?}",idx);	
					
			crossbeam::scope(|scope| {
				let c = ch;
				let starts = if idx == 0 { 1 } else { 0 };
			 	let ends = if idx == pattern_chars.len()-1 { 1 } else { 0 };
			 	//let mut facts = vec![];
			 	
				for v in &self.facts {
					//println!("list number {:?}", v.len());
					scope.spawn(move || {					
						for value in v {
							if value.starts_with == starts && value.ends_with == ends && value.pattern_placeholder == *c {
								println!("pattern symbol={:?}, starts={:?}, ends={:?}, key {:?}", *c, starts, ends, value.key);
								//facts.push(value.key);
							}
						}
					});
				}
				
				//println!("pattern symbol={:?}, starts={:?}, ends={:?}, key {:?}", *c, starts, ends, facts);
			}); 
		
		}
		
		Vec::new()
	}

	pub fn reset_analyze(&mut self) {
		self.patterns = PatternMap::new();
	}
}