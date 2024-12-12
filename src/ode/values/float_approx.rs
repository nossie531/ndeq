//! Provider of [`FloatApprox`].

/// This trait represents that a type can be approximated by [`f32`].
pub trait FloatApprox {
    /// Converts this type into [`f32`].
    fn approx_into_float(self) -> f32;

    /// Converts to this type from [`f32`].
    fn approx_from_float(x: f32) -> Self;
}

impl FloatApprox for f32 {
    fn approx_into_float(self) -> f32 {
        self
    }

    fn approx_from_float(x: f32) -> Self {
        x
    }
}

impl FloatApprox for f64 {
    fn approx_into_float(self) -> f32 {
        self as f32
    }

    fn approx_from_float(x: f32) -> Self {
        x as Self
    }
}
