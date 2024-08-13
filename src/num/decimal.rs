use crate::{Answer, Calculation, Context, MathError, Num};
use rust_decimal::{Decimal, MathematicalOps};
use std::cmp::Ordering;
use std::f64;

impl Num for Decimal {
	fn from_f64(t: f64, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(Decimal::from_f64_retain(t).unwrap()))
	}

	fn from_f64_complex((r, _i): (f64, f64), _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(Decimal::from_f64_retain(r).unwrap()))
	}

	fn typename() -> String {
		String::from("decimal")
	}

	/// Compares two floats. Errors if either is NaN. Infinity is greater than anything except equal
	/// to infinity. Negative infinity is less than anything except equal to negative infinity.
	fn tryord(&self, other: &Self, _ctx: &Context<Self>) -> Result<Ordering, MathError> {
		Ok(self.cmp(other))
	}

	fn add(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(*self + *other))
	}

	fn sub(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(*self - *other))
	}

	fn mul(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(*self * *other))
	}

	fn div(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		if *other == Self::ZERO {
			return Err(MathError::DivideByZero);
		}

		Ok(Answer::Single(*self / *other))
	}

	fn pow(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(self.powd(*other)))
	}

	fn sqrt(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(MathematicalOps::sqrt(self).unwrap()))
	}

	fn abs(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(self.abs()))
	}

	fn sin(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(MathematicalOps::sin(self)))
	}

	fn cos(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(MathematicalOps::cos(self)))
	}

	fn tan(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(MathematicalOps::tan(self)))
	}

	fn asin(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Err(MathError::Unimplemented { op: "Asin".to_string(), num_type: Self::typename() })
	}

	fn acos(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Err(MathError::Unimplemented { op: "Acos".to_string(), num_type: Self::typename() })
	}

	fn atan(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Err(MathError::Unimplemented { op: "Atan".to_string(), num_type: Self::typename() })
	}

	fn atan2(&self, _other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		Err(MathError::Unimplemented { op: "Atan2".to_string(), num_type: Self::typename() })
	}

	fn floor(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(self.floor()))
	}

	fn ceil(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(self.ceil()))
	}

	fn round(&self, _ctx: &Context<Self>) -> Calculation<Self> {
		Ok(Answer::Single(self.round()))
	}

	fn log(&self, other: &Self, _ctx: &Context<Self>) -> Calculation<Self> {
		let ln_value = MathematicalOps::ln(self);
		let ln_base = MathematicalOps::ln(other);
		Ok(Answer::Single(ln_value / ln_base))
	}
}
