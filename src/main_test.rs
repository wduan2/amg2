#[cfg(test)]
mod main_test {
    use crate::{execute_with_args, vec_of_strings};

    #[test]
    #[should_panic]
    fn test_execute_with_unknown_command() {
        execute_with_args(vec_of_strings!("target/amg2", "unknown"));
    }

    #[test]
    fn test_create_happy_case() {
        execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--website",
            "abc",
            "--username",
            "test@gmail.com",
            "--password",
            "123"
        ));
    }

    #[test]
    #[should_panic]
    fn test_create_missing_required_arg() {
        execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--password",
            "123"
        ));
    }

    #[test]
    #[should_panic]
    fn test_create_missing_arg_value() {
        execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--website",
            "--username",
            "--password",
            "123"
        ));
    }

    #[test]
    #[should_panic]
    fn test_create_with_unknown_arg() {
        execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--wrong-arg",
            "abc",
            "--username",
            "test@gmail.com",
            "--password",
            "123"
        ));
    }
}
