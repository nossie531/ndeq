//! Provider of [`WorkOn`].

/// Work place.
///
/// This type allows short coding around compound assignments by using fulunet
/// interface. (Compound assignment is memory efficient rather than assignments
/// and binary operators. This is important when operator overloading is used
/// for very large size type.)
pub struct WorkOn<'a, T>(pub &'a mut T);

impl<'a, T> WorkOn<'a, T> {
    /// Sets value of this instance.
    pub fn set(self, value: &T) -> Self
    where
        T: Clone,
    {
        self.0.clone_from(value);
        self
    }

    /// Execute mutation callback and returns result reference.
    pub fn calc<F>(self, mut f: F) -> &'a mut T
    where
        F: FnMut(&mut T),
    {
        f(self.0);
        self.0
    }
}
