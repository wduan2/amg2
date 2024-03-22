#[macro_export]
macro_rules! vec_of_strings {
    ( $( $str:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($str.to_string());
            )*
            v
        }
    };
}
