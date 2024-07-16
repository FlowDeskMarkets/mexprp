use std::collections::HashMap;
use std::fmt::Debug;
use crate::num::Num;

/// A trait for adapter to update and access supplementary data
pub trait SupplementaryDataAdapter<N: Num>: Debug {
	/// Retrieve the supplementary data
	fn get(&self, key: &str) -> Option<&Vec<SupplementaryDataType<N>>>;
	/// Update the supplementary data
	fn set(&mut self, key: String, value: SupplementaryDataType<N>);
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
	Map(HashMap<String, SupplementaryDataType<N>>)
}