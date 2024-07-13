use std::collections::HashMap;
use crate::num::Num;

/// A supplementary struct holds the value that cannot be used directly in expression evaluation
#[derive(Clone)]
pub struct Supplementary<N: Num> {
	/// HashMap of variables
	pub vectors: HashMap<String, Vec<SupplementaryDataType<N>>>,
	/// HashMap of functions
	pub map: HashMap<String, HashMap<String, SupplementaryDataType<N>>>
}

impl <N: Num> Supplementary<N> {
	/// Create a new Supplementary
	pub fn new() -> Self {
		Supplementary {
			vectors: HashMap::new(),
			map: HashMap::new()
		}
	}

	pub fn add_vector(&mut self, key: String, value: SupplementaryDataType<N>) {
		let current = self.vectors.get(&key);
		if current.is_some() && current.unwrap().len() > 0 {
			let mut current = current.unwrap().clone();
			current.push(value);
			self.vectors.insert(key, current);
		} else {
			self.vectors.insert(key, vec![value]);
		}
	}

	pub fn add_hashmap(&mut self, key: String, value: HashMap<String, SupplementaryDataType<N>) {
		self.map.insert(key, value);
	}
}

/// Supported Data Type for Supplementary
#[derive(Clone)]
pub enum SupplementaryDataType<N: Num> {
	/// Numeric value
	Num(N),
	/// String value
	String(String),
}