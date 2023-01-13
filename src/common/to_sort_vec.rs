pub trait ToSortVec<A> {
    fn to_sort_vec(self) -> Vec<A>;
}

impl<A, I1> ToSortVec<A> for I1
where
    I1: IntoIterator<Item = A>,
    A: Ord,
{
    fn to_sort_vec(self) -> Vec<A> {
        let mut vec: Vec<A> = self.into_iter().collect();
        vec.sort();
        vec
    }
}
