#[macro_export]
macro_rules! sort_test {
    ( $name:ident, $file:expr ) => {
        #[test]
        fn $name() -> Result<(), failure::Error> {
            let input = if let Ok(input) = std::fs::read_to_string($file) {
                input
            } else {
                return Err(failure::err_msg(format!(
                    "Failed to open test file `{}`",
                    $file
                )));
            };

            let exp = if let Ok(exp) = std::fs::read_to_string(format!("{}.sort.exp", $file)) {
                exp
            } else {
                return Err(failure::err_msg(format!(
                    "Failed to open exp file `{}.sort.exp`",
                    $file
                )));
            };

            let mut buffer = Vec::new();
            use crate::rewrite;
            rewrite::rewrite(None, &input, &mut buffer)?;

            let output = std::str::from_utf8(&buffer)?;

            pretty_assertions::assert_eq!(&exp, &output);

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! next_test {
    ( $name:ident, $file:expr ) => {
        #[test]
        fn $name() -> Result<(), failure::Error> {
            let input = if let Ok(input) = std::fs::read_to_string($file) {
                input
            } else {
                return Err(failure::err_msg(format!(
                    "Failed to open test file `{}`",
                    $file
                )));
            };

            let exp = if let Ok(exp) = std::fs::read_to_string(format!("{}.next.exp", $file)) {
                exp
            } else {
                return Err(failure::err_msg(format!(
                    "Failed to open exp file `{}.next.exp`",
                    $file
                )));
            };

            let mut buffer = Vec::new();
            use crate::rewrite;
            rewrite::rewrite(Some("Today"), &input, &mut buffer)?;

            let output = std::str::from_utf8(&buffer)?;

            pretty_assertions::assert_eq!(&exp, &output);

            Ok(())
        }
    };
}
