use test_data_generator::profile::pattern::{Pattern};
use std::collections::BTreeMap;
use std::ops::AddAssign;
//use std::iter::Sum;

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
		}
	}
	
	// methods
	pub fn analyze(&mut self, entity: &str) {
		let mut pattrn =  Pattern::new();
		
		// analyze patterns
		//let mut rslt = &pattrn.analyze(entity);
		AddAssign::add_assign(self.patterns.entry(pattrn.analyze(entity).0.to_string()).or_insert(0), 1);
		self.pattern_total = self.patterns.values().sum::<u32>();
		//println!("{:?}",&self.patterns);
		//println!("{:?}",self.pattern_total);
		
		// analyze sizes
		AddAssign::add_assign(self.sizes.entry(pattrn.size).or_insert(0), 1);
		self.size_total = self.sizes.values().sum::<u32>();
	} 
	
	pub fn rank_patterns(&mut self) -> PatternRankMap{
		self.pattern_ranks = PatternRankMap::new();
		
		for key in self.patterns.keys(){
			self.pattern_ranks.insert(key.to_string(), (*self.patterns.get(key).unwrap() as f64 / self.pattern_total as f64)*100.0);
		}
		
		println!("{:?}",&self.pattern_ranks);
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
		
		println!("{:?}",&self.size_ranks);
		self.size_ranks.clone()
	} 
}