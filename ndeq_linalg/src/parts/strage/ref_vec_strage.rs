
/// External referencing matrix strage with vector.
pub struct RefVecStrage<'a, T> {
    pub(crate) vec: &'a T,
}
