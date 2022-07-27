pub trait ToSortVecVec<A> {
    fn to_sort_vec_vec(self) -> Vec<Vec<A>>;
}

impl<A, I1, I2> ToSortVecVec<A> for I2
where
    I2: IntoIterator<Item = I1>,
    I1: IntoIterator<Item = A>,
    A: Ord,
{
    fn to_sort_vec_vec(self) -> Vec<Vec<A>> {
        let mut vec: Vec<Vec<A>> = self
            .into_iter()
            .map(|e| {
                let mut vec: Vec<A> = e.into_iter().collect();
                vec.sort();
                vec
            })
            .collect();
        vec.sort();
        vec
    }
}
