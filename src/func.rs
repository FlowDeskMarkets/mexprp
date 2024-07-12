use crate::term::Term;
use crate::context::Context;
use crate::opers::Calculation;
use crate::num::Num;
use crate::Supplementary;

/// Implemented by functions defined in a context
pub trait Func<N: Num> {
	/// Evaluate the function in this context with the given arguments. When implementing,
	/// simply evaluate the arguments with the context and return an `Err(MathError::IncorrectArguments)`
	/// if there are too many or too few.
	fn eval(&self, args: &[Term<N>], ctx: &Context<N>, supp: Option<&Supplementary<N>>) -> Calculation<N>;
}

/// Blanket impl for closures
impl<T, N: Num> Func<N> for T
where
	T: Fn(&[Term<N>], &Context<N>, Option<&Supplementary<N>>) -> Calculation<N>,
{
	fn eval(&self, args: &[Term<N>], ctx: &Context<N>, supp: Option<&Supplementary<N>>) -> Calculation<N> {
		self(args, ctx, supp)
	}
}
