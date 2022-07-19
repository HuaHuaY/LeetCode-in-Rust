pub trait ToVecVec<A> {
    fn to_vec_vec(self) -> Vec<Vec<A>>;
}

impl<A, const L1: usize, const L2: usize> ToVecVec<A> for [[A; L1]; L2]
where
    A: Clone,
{
    fn to_vec_vec(self) -> Vec<Vec<A>> {
        IntoIterator::into_iter(self).map(|e| e.to_vec()).collect()
    }
}
