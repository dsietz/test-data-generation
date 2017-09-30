use test_data_generator::profile::pattern::{Pattern};
use test_data_generator::profile::fact::{Fact};
use std::collections::BTreeMap;
use std::ops::AddAssign;

type PatternMap = BTreeMap<String, u32>;
type PatternRankMap  = BTreeMap<String, f64>;
type SizeMap = BTreeMap<u32, u32>;
type SizeRankMap  = BTreeMap<u32, f64>;

pub struct Profile{
	pub patterns: PatternMap,
	pub pattern_total: u32,
	pub pattern_ranks: PatternRankMap,
	pub sizes: SizeMap,
	pub size_total: u32,
	pub size_ranks: SizeRankMap,
	pub processors: u8,
	pub facts: Vec<Vec<Fact>>,
}

impl Profile {
	//constructor
	pub fn new() -> Profile {
		Profile{
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_ranks: PatternRankMap::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: SizeRankMap::new(), 
			processors: 4,
			facts: Profile::new_facts(4),
		}
	}
	
	pub fn new_with(p: u8) -> Profile {
		Profile{
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_ranks: PatternRankMap::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: SizeRankMap::new(), 
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
	} 
	
	pub fn cum_sizemap(&mut self) {
		for key in self.sizes.keys(){
			self.size_ranks.insert(*key, (*self.sizes.get(key).unwrap() as f64 / self.size_total as f64)*100.0);
		}
	}
	
	pub fn pre_generate(&mut self){
		self.cum_sizemap();
	}
	
	pub fn generate(&mut self) -> bool{
		// first, determine the length of the entity
	 	let mut sizes = self.size_ranks.iter().collect::<Vec<_>>();
	 	sizes.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
	 	let mut s: f64 = 0 as f64;
	 	random_percentage!(s);

println!("rand: {}", s);
println!("sizes: {:?}", sizes);

		 	
	 	
		
		// second, determine the pattern to use
		
		// build the entity using facts that adhere to the pattern 
		
		
		true
	}
	
	fn new_facts(p: u8) -> Vec<Vec<Fact>> {
		let mut vec_main = Vec::new();
		
		for _ in 0..p {  
			vec_main.push(Vec::new());
		}

		vec_main
	}

	pub fn rank_patterns(&mut self) -> PatternRankMap{
		self.pattern_ranks = PatternRankMap::new();
		
		for key in self.patterns.keys(){
			self.pattern_ranks.insert(key.to_string(), (*self.patterns.get(key).unwrap() as f64 / self.pattern_total as f64)*100.0);
		}
		
		self.pattern_ranks.clone()
	}
	
	pub fn reset_analyze(&mut self) {
		self.patterns = PatternMap::new();
	}
	
	pub fn rank_sizes(&mut self) -> SizeRankMap{
		self.size_ranks = SizeRankMap::new();

		for key in self.sizes.keys(){
			self.size_ranks.insert(*key, (*self.sizes.get(key).unwrap() as f64 / self.size_total as f64)*100.0);
		}
		
		//println!("{:?}",&self.size_ranks);
		self.size_ranks.clone()
	} 
}




// How to sort a Vec
// v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));