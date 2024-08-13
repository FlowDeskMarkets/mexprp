use crate::num::ComplexFloat;
use crate::{SupplementaryDataAdapter, SupplementaryDataType};
use rug::Rational;
use std::collections::HashMap;

/// Mock adapter for tests
#[derive(Debug)]
pub struct MockSupplementaryDataAdapter {
	buffers_f64: HashMap<String, Vec<SupplementaryDataType<f64>>>,
	buffers_complex_float: HashMap<String, Vec<SupplementaryDataType<ComplexFloat>>>,
	buffers_rational: HashMap<String, Vec<SupplementaryDataType<Rational>>>,
}

impl MockSupplementaryDataAdapter {
	/// To instantiate a new MockSupplementaryDataAdapter
	pub fn new() -> Self {
		MockSupplementaryDataAdapter {
			buffers_f64: HashMap::new(),
			buffers_complex_float: HashMap::new(),
			buffers_rational: HashMap::new(),
		}
	}
}
impl Default for MockSupplementaryDataAdapter {
	fn default() -> Self {
		Self::new()
	}
}
impl SupplementaryDataAdapter<f64> for MockSupplementaryDataAdapter {
	fn get(&self, key: &str) -> Option<&Vec<SupplementaryDataType<f64>>> {
		self.buffers_f64.get(key)
	}
	fn push(&mut self, key: String, value: SupplementaryDataType<f64>) {
		if let Some(buffer) = self.buffers_f64.get_mut(&key) {
			buffer.push(value);
		} else {
			self.buffers_f64.insert(key, vec![value]);
		}
	}
	fn prune(&mut self, key: String, length: usize) {
		if let Some(buffer) = self.buffers_f64.get_mut(&key) {
			if length <= buffer.len() {
				buffer.drain(0..length);
			}
		}
	}
	fn set(&mut self, key: String, vec: &Vec<SupplementaryDataType<f64>>) {
		self.buffers_f64.insert(key, vec.clone());
	}
}
impl SupplementaryDataAdapter<ComplexFloat> for MockSupplementaryDataAdapter {
	fn get(&self, key: &str) -> Option<&Vec<SupplementaryDataType<ComplexFloat>>> {
		self.buffers_complex_float.get(key)
	}
	fn push(&mut self, key: String, value: SupplementaryDataType<ComplexFloat>) {
		if let Some(buffer) = self.buffers_complex_float.get_mut(&key) {
			buffer.push(value);
		} else {
			self.buffers_complex_float.insert(key, vec![value]);
		}
	}
	fn prune(&mut self, key: String, length: usize) {
		if let Some(buffer) = self.buffers_complex_float.get_mut(&key) {
			if length <= buffer.len() {
				buffer.drain(0..length);
			}
		}
	}
	fn set(&mut self, key: String, vec: &Vec<SupplementaryDataType<ComplexFloat>>) {
		self.buffers_complex_float.insert(key, vec.clone());
	}
}
impl SupplementaryDataAdapter<Rational> for MockSupplementaryDataAdapter {
	fn get(&self, key: &str) -> Option<&Vec<SupplementaryDataType<Rational>>> {
		self.buffers_rational.get(key)
	}
	fn push(&mut self, key: String, value: SupplementaryDataType<Rational>) {
		if let Some(buffer) = self.buffers_rational.get_mut(&key) {
			buffer.push(value);
		} else {
			self.buffers_rational.insert(key, vec![value]);
		}
	}
	fn prune(&mut self, key: String, length: usize) {
		if let Some(buffer) = self.buffers_rational.get_mut(&key) {
			if length <= buffer.len() {
				buffer.drain(0..length);
			}
		}
	}
	fn set(&mut self, key: String, vec: &Vec<SupplementaryDataType<Rational>>) {
		self.buffers_rational.insert(key, vec.clone());
	}
}
