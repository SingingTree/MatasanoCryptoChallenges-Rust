use std::num::Float;

pub trait ApproxEquality<T> {
	fn approx_equal(self, other: &T) -> bool;
}

impl ApproxEquality<f32> for f32 {
	fn approx_equal(self, other : &f32) -> bool {
		let epsilon : f32 = 0.0001;
		return self - *other < epsilon;
	}
}

impl ApproxEquality<f64> for f64 {
	fn approx_equal(self, other : &f64) -> bool {
		let epsilon : f64 = 0.0001;
		return self - *other < epsilon;
	}
}