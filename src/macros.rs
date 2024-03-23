use std::collections::HashMap;

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

#[macro_export]
macro_rules! arg_option_map {
    ( $( $name:expr, $required:expr, $has_value:expr ),* ) => {
        {
            let mut m = HashMap::new();
            $(
                m.insert(String::from($name), CommandArgOption::new($name, $required, $has_value));
            )*
            m
        }
    };
}
