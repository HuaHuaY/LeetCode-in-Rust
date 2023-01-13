use super::ToSortVec;

pub trait ToSortVecVec<A> {
    fn to_sort_vec_vec(self) -> Vec<Vec<A>>;
}

impl<A, B, I1> ToSortVecVec<A> for I1
where
    I1: IntoIterator<Item = B>,
    B: ToSortVec<A>,
    A: Ord,
{
    fn to_sort_vec_vec(self) -> Vec<Vec<A>> {
        let mut vec: Vec<Vec<A>> = self.into_iter().map(ToSortVec::to_sort_vec).collect();
        vec.sort();
        vec
    }
}
