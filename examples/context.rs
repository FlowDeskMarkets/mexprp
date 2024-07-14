//! This example shows how to evaluate an expression with a variable and function in the context of
//! the expression.

extern crate mexprp;

use mexprp::{Answer, Calculation, Context, Expression, MathError, Num, SupplementaryDataAdapter, Term};

fn main() {
	// A context holds data that can be used in an expression
	let mut context: Context<f64> = Context::new();
	// Add a variable "x" to the context with the value 5.4
	context.set_var("x", 5.4);
	// Add a function "sum" to the context that returns the sum of it's arguments. A closure is passed
	// in that takes twp arguments: args: &[Term], which is a slice if the arguments passed into the
	// function, and ctx: &Context, which is a reference to the context which the expression is being
	// evaluated with. The item passed in can be anything that implements the `Func` trait. There exists
	// a blanket impl for Fn(&[Term], &Context) -> Calculation which allows you to pass in closures in
	// that format.
	context.set_func(
		"sum",
		|args: &[Term<f64>], ctx: &Context<f64>, _supp: Option<Box<dyn SupplementaryDataAdapter<f64>>>| -> Calculation<f64> {
			if args.len() < 1 {
				return Err(MathError::IncorrectArguments);
			};

			let mut sum = Answer::Single(0.0);
			for arg in args {
				let b = arg.eval_ctx(ctx, None)?;
				sum = sum.op(&b, |a, b| Num::add(a, b, ctx))?;
			}

			Ok(sum)
		},
	);

	let raw = "2 * sum(x, 7, 400)";
	// The expression needs to be parsed with the context in order do decide if some names are functions
	// or variables.
	let expr = Expression::parse_ctx(raw, context).unwrap();
	// The expression also needs to be evaluated with a context. This context can be different than the
	// one it was parsed with, but if it is missing something that is necessary for evaluation the
	// evaluation will fail.
	println!("{} = {}", raw, expr.eval(None).unwrap())
}
