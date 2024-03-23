#[cfg(test)]
mod main_test {
    use crate::{execute_with_args, vec_of_strings};

    #[test]
    fn test_execute_with_unknown_command() {
        let r = execute_with_args(vec_of_strings!("target/amg2", "unknown"));
        assert_eq!(true, r.is_err());
        assert!(r.err().unwrap().to_string().contains("Unknown command"));
    }

    #[test]
    fn test_create_happy_case() {
        let r = execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--website",
            "abc",
            "--username",
            "test@gmail.com",
            "--password",
            "123"
        ));
        assert_eq!(false, r.is_err());
    }

    #[test]
    fn test_create_missing_required_arg() {
        let r = execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--password",
            "123"
        ));
        assert_eq!(true, r.is_err());
        assert!(r.err().unwrap().to_string().contains("Missing required argument"));
    }

    #[test]
    fn test_create_missing_arg_value() {
        let r = execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--website",
            "--username",
            "--password",
            "123"
        ));
        assert_eq!(true, r.is_err());
        assert!(r.err().unwrap().to_string().contains("Missing value for argument"));
    }

    #[test]
    fn test_create_with_unknown_arg() {
        let r = execute_with_args(vec_of_strings!(
            "target/amg2",
            "create",
            "--wrong-arg",
            "abc",
            "--username",
            "test@gmail.com",
            "--password",
            "123"
        ));
        assert_eq!(true, r.is_err());
        assert!(r.err().unwrap().to_string().contains("Unknown argument"));
    }
}
