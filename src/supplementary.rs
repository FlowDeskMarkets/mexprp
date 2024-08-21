use crate::num::Num;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

/// A trait for adapter to update and access supplementary data
pub trait SupplementaryDataAdapter<N: Num>: Debug {
	/// Retrieve the supplementary data
	fn get(&self, key: &str) -> Option<&Vec<SupplementaryDataType<N>>>;
	/// Add new supplementary data
	fn push(&mut self, key: String, value: SupplementaryDataType<N>);
	/// Optional; remove first X data items
	fn prune(&mut self, _key: String, _length: usize) {}
	/// Optional; replace entire data set
	fn set(&mut self, _key: String, _vec: &Vec<SupplementaryDataType<N>>) {}
}

/// Supported Data Type for Supplementary
#[derive(Clone, Debug)]
pub enum SupplementaryDataType<N: Num> {
	/// Numeric value
	Num(N),
	/// String value
	String(String),
	/// Vector
	Vector(Vec<SupplementaryDataType<N>>),
	/// Map
	Map(HashMap<String, SupplementaryDataType<N>>),
	/// Double-ended queue
	Deque(VecDeque<SupplementaryDataType<N>>),
}
