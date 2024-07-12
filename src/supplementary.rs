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

/// Supported Data Type for Supplementary
#[derive(Clone)]
pub enum SupplementaryDataType<N: Num> {
	/// Numeric value
	Num(N),
	/// String value
	String(String),
}