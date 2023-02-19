pub trait ToVecChar {
    fn to_vec_char(self) -> Vec<char>;
}

impl<'a, I1> ToVecChar for I1
where
    I1: IntoIterator<Item = &'a str>,
{
    fn to_vec_char(self) -> Vec<char> {
        self.into_iter().flat_map(|e| e.chars()).collect()
    }
}
