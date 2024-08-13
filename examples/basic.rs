//! This example shows a simple command line calculator written with this library, with some basic
//! error handling.

extern crate mexprp;

use mexprp::mock_supplementary_data_adapter::MockSupplementaryDataAdapter;
use mexprp::{Expression, SupplementaryDataAdapter};
use std::io::{self, Write};
use std::sync::{Arc, RwLock};

fn main() {
	println!("MEXPRP Test Calculator\n---------------------");
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	loop {
		let mut buf = String::new();
		print!("> ");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut buf).unwrap();

		// Parse the expression (with the default context)
		let expr: Expression<f64> = match Expression::parse(&buf) {
			Ok(expr) => expr,
			Err(e) => {
				println!("Failed to parse expression: {}", e);
				continue;
			}
		};

		// Evaluate the expression or print the error if there was one
		match expr.eval(Arc::clone(&supp)) {
			Ok(val) => println!("\t= {}", val),
			Err(e) => println!("Failed to evaluate the expression: {}", e),
		}
	}
}
