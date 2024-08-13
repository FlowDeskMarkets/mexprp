use crate::mock_supplementary_data_adapter::MockSupplementaryDataAdapter;
use crate::num::ComplexFloat;
use crate::supplementary::SupplementaryDataAdapter;
use crate::{eval, Answer, Calculation, Context, Expression, Num, Term};
use std::sync::{Arc, RwLock};

#[test]
fn plain() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let a: f64 = eval("2 * 8.5 + -9", Arc::clone(&supp)).unwrap().unwrap_single();
	assert!(eq(a, 8.0));
	let a: f64 = eval("sin(pi)--5 ^ 2", Arc::clone(&supp)).unwrap().unwrap_single();
	assert!(eq(a, 25.0));
}

#[test]
fn basic() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let raw = "(3 * (17.8 - 4) ^ 2) / 7";
	println!("\nParsing {}", raw);
	let expr: Expression<f64> = Expression::parse(raw).unwrap();
	println!("\nDone parsing {}", raw);
	assert!((expr.eval(supp).unwrap().unwrap_single() - 81.61714285714285).abs() < 0.001);
}

#[test]
fn var_context() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let expr: Expression<f64> = Expression::parse("3 - x ^ (0-3 + 0.22)").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("x", 7.0);
	ctx.set_var("y", 0.22);
	assert!((expr.eval_ctx(&ctx, supp).unwrap().unwrap_single() - 2.995526705934608).abs() < 0.001);
}

#[test]
fn expr_context() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<ComplexFloat>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let expr: Expression<ComplexFloat> = Expression::parse("3 * something").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("something", Expression::parse("(0-8) * 2").unwrap());
	assert_eq!(expr.eval_ctx(&ctx, supp).unwrap().unwrap_single().r, -48.0);
}

#[test]
fn funky() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<ComplexFloat>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let expr: Expression<ComplexFloat> = Expression::parse("3(x * -(3 + 1))").unwrap();
	let mut ctx = Context::new();
	ctx.set_var("x", ComplexFloat::from(2.0));
	assert_eq!(expr.eval_ctx(&ctx, supp).unwrap().unwrap_single().r, -24.0);
}

#[test]
fn sin() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	let expr: Expression<f64> = Expression::parse("2 + sin(3.1415926)").unwrap();
	assert!((expr.eval(supp).unwrap().unwrap_single() - 2.0) < 0.005);
}

#[test]
fn funcs() {
	let supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>> = Arc::new(RwLock::new(MockSupplementaryDataAdapter::default()));
	assert!(eq(eval::<f64>("max(sin(2), 5000000, -4)", Arc::clone(&supp)).unwrap().unwrap_single(), 5000000.0));
	assert!(eq(eval::<f64>("min(2 / -3 * 3 * 3, 5000000, -4)", Arc::clone(&supp)).unwrap().unwrap_single(), -6.0));
	let mut context: Context<f64> = Context::new();
	context.set_func("sum", |args: &[Term<f64>], ctx: &Context<f64>, supp: Arc<RwLock<dyn SupplementaryDataAdapter<f64>>>| -> Calculation<f64> {
		let mut x = Answer::Single(0.0);
		for arg in args {
			let a = arg.eval_ctx(ctx, Arc::clone(&supp))?;
			x = x.op(&a, |a, b| Num::add(a, b, ctx))?;
		}
		Ok(x)
	});
	let expr: Expression<f64> = Expression::parse_ctx("sum(4, 5, 6) / 3", context).unwrap();
	assert!(eq(expr.eval(supp).unwrap().unwrap_single(), 5.0));
}

fn eq<N: Num + 'static>(x: N, y: f64) -> bool {
	use std::cmp::Ordering;
	let ctx = &Context::empty();
	x.sub(&N::from_f64(y, ctx).unwrap().unwrap_single(), ctx)
		.unwrap()
		.unwrap_single()
		.abs(ctx)
		.unwrap()
		.unwrap_single()
		.tryord(&N::from_f64(0.00001, ctx).unwrap().unwrap_single(), ctx)
		.unwrap()
		== Ordering::Less
}
