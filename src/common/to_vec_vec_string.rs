use super::ToVecString;

pub trait ToVecVecString {
    fn to_vec_vec_string(self) -> Vec<Vec<String>>;
}

impl<A, I1> ToVecVecString for I1
where
    I1: IntoIterator<Item = A>,
    A: ToVecString,
{
    fn to_vec_vec_string(self) -> Vec<Vec<String>> {
        self.into_iter().map(|e| e.to_vec_string()).collect()
    }
}

#[macro_export]
macro_rules! vec_vec_string {
    [$([$($e:expr), *]), *] => {
        vec![$(vec![$($e.into()), *]), *]
    };
}
