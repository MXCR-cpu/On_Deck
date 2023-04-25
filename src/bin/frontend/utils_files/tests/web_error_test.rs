#[cfg(test)]
mod tests {
    mod web_error {
        use utils_files::web_error::ClientError;

        #[test]
        fn test_constructor() {
            assert_eq!(ClientError::new(), ClientError { stack: Vec::new() });
        }

        #[test]
        fn test_from() {
            assert_eq!(
                "tests/web_error_test.rs: This is a test error using &str",
                format!(
                    "{}",
                    ClientError::from(file!(), "This is a test error using &str")
                )
            );
            assert_eq!(
                "tests/web_error_test.rs: This is a test error using String",
                format!(
                    "{}",
                    ClientError::from(file!(), &"This is a test error using String".to_string())
                )
            );
        }
    }
}
