pub trait ToVecVec<A> {
    fn to_vec_vec(self) -> Vec<Vec<A>>;
}

impl<A, I1, I2> ToVecVec<A> for I2
where
    I2: IntoIterator<Item = I1>,
    I1: IntoIterator<Item = A>,
{
    fn to_vec_vec(self) -> Vec<Vec<A>> {
        self.into_iter().map(|e| e.into_iter().collect()).collect()
    }
}
