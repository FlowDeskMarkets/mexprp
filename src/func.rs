use crate::context::Context;
use crate::num::Num;
use crate::opers::Calculation;
use crate::supplementary::SupplementaryDataAdapter;
use crate::term::Term;
use std::sync::{Arc, RwLock};

/// Implemented by functions defined in a context
pub trait Func<N: Num> {
	/// Evaluate the function in this context with the given arguments. When implementing,
	/// simply evaluate the arguments with the context and return an `Err(MathError::IncorrectArguments)`
	/// if there are too many or too few.
	fn eval(&self, args: &[Term<N>], ctx: &Context<N>, supp: Arc<RwLock<dyn SupplementaryDataAdapter<N>>>) -> Calculation<N>;
}

/// Blanket impl for closures
impl<T, N: Num> Func<N> for T
where
	T: Fn(&[Term<N>], &Context<N>, Arc<RwLock<dyn SupplementaryDataAdapter<N>>>) -> Calculation<N>,
{
	fn eval(&self, args: &[Term<N>], ctx: &Context<N>, supp: Arc<RwLock<dyn SupplementaryDataAdapter<N>>>) -> Calculation<N> {
		self(args, ctx, supp)
	}
}
