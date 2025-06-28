//! Provider of [`Work`].

/// Short coding helper with work.
pub struct Work<'a, T: Clone>(
    /// Working place.
    pub &'a mut T,
    /// Original data.
    pub &'a T,
);

impl<'a, T: Clone> Work<'a, T> {
    /// Execute method on copied work.
    pub fn exec<F>(self, mut f: F) -> &'a mut T
    where
        F: FnMut(&mut T),
    {
        self.0.clone_from(self.1);
        f(self.0);
        self.0
    }
}
