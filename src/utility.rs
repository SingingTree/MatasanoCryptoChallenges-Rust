use std::num::Float;

pub trait ApproxEquality<T> {
	fn approx_equal(self, other: T) -> bool;
}

impl ApproxEquality<f32> for f32 {
	fn approx_equal(self, other : f32) -> bool {
		let epsilon : f32 = 0.000000001;
		return self - other < epsilon;
	}
}

impl ApproxEquality<f64> for f64 {
	fn approx_equal(self, other : f64) -> bool {
		let epsilon : f64 = 0.000000001;
		return self - other < epsilon;
	}
}


#[cfg(test)]
mod tests {
	use utility::ApproxEquality;

	#[test]
	fn f32_approx_equal_test() {
		let num1 : f32 = 0.001;
		let num2 : f32 = 0.0010000000001;

		assert!(num1.approx_equal(num2));

	}

	#[test]
	fn f32_approx_not_equal_test() {
		let num1 : f32 = 0.001;
		let num2 : f32 = 0.00101;

		assert!(!num1.approx_equal(num2));
	}
}