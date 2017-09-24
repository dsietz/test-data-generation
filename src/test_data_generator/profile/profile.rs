use test_data_generator::profile::pattern::{Pattern};
use std::collections::BTreeMap;
use std::ops::AddAssign;
//use std::iter::Sum;

type CountMap = BTreeMap<String, u32>;
type RankMap  = BTreeMap<String, f64>;


pub struct Profile{
	pub patterns: CountMap,
	pub pattern_total: u32,
	pub pattern_ranks: RankMap,
}

impl Profile {
	//constructor
	pub fn new() -> Profile {
		Profile{
			patterns: CountMap::new(),
			pattern_total: 0,
			pattern_ranks: RankMap::new(),
		}
	}
	
	// methods
	pub fn analyze(&mut self, entity: &str) {
		let mut pattrn =  Pattern::new();
		AddAssign::add_assign(self.patterns.entry(pattrn.analyze(entity).to_string()).or_insert(0), 1);
		self.pattern_total = self.patterns.values().sum::<u32>();
		//println!("{:?}",&self.patterns);
		//println!("{:?}",self.pattern_total);
	} 
	
	pub fn rank(&mut self) -> RankMap{
		self.pattern_ranks = RankMap::new();
		
		for key in self.patterns.keys(){
			self.pattern_ranks.insert(key.to_string(), (*self.patterns.get(key).unwrap() as f64 / self.pattern_total as f64)*100.0);
		}
		
		println!("{:?}",&self.pattern_ranks);
		self.pattern_ranks.clone()
	}
	
	pub fn reset_analyze(&mut self) {
		self.patterns = CountMap::new();
	} 
}