use super::ToVecChar;

pub trait ToVecVecChar {
    fn to_vec_vec_char(self) -> Vec<Vec<char>>;
}

impl<A, I1> ToVecVecChar for I1
where
    I1: IntoIterator<Item = A>,
    A: ToVecChar,
{
    fn to_vec_vec_char(self) -> Vec<Vec<char>> {
        self.into_iter().map(|e| e.to_vec_char()).collect()
    }
}
