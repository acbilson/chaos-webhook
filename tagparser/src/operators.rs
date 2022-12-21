use std::ffi::OsStr;

pub fn is_markdown(file_name: &OsStr) -> bool {
    return file_name.to_string_lossy().ends_with(".md");
}

#[cfg(test)]
mod operator_tests {
    use std::ffi::OsStr;

    use crate::operators;

    #[test]
    fn is_markdown_correctly_matches_file() {
        // arrange
        let file_name = OsStr::new("test.md");

        // act
        let result = operators::is_markdown(&file_name);

        // assert
        assert_eq!(result, true);
    }
}
