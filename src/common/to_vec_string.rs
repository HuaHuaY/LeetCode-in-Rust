pub trait ToVecString {
    fn to_vec_string(self) -> Vec<String>;
}

impl<A, I1> ToVecString for I1
where
    I1: IntoIterator<Item = A>,
    A: Into<String>,
{
    fn to_vec_string(self) -> Vec<String> {
        self.into_iter().map(|e| e.into()).collect()
    }
}
