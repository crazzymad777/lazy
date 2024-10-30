pub mod builder;
pub mod parser;
mod utils;

pub use crate::omicron::command::builder::CommandBuilder;
pub use crate::omicron::command::parser::CommandParser;
pub use crate::omicron::command::parser::parse;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parser_empty() {
        let expected = CommandBuilder::new().to_string();
        let actual = parse("").to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parser_args() {
        let expected = CommandBuilder::new()
            .program("hello\0")
            .arg("1\0")
            .arg("2\0")
            .arg("3\0")
            .to_string();
        let actual = parse("hello 1 2 3").to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parser_single_quotes() {
        let expected = CommandBuilder::new()
            .program("hello\0")
            .arg("1 2 3\0")
            .arg("4 5 6\0")
            .to_string();
        let actual = parse("hello '1 2 3' '4 5 6'").to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parser_escape_single_quote() {
        let expected = CommandBuilder::new()
            .program("hello\0")
            .arg("'1\0")
            .arg("2\0")
            .arg("3'\0")
            .arg("4 5 6\0")
            .to_string();
        let actual = parse("hello \\'1 2 3\\' '4 5 6'").to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parser_escape_in_single_quotes() {
        let expected = CommandBuilder::new()
            .program("hello\0")
            .arg("1 2 3\\\0")
            .to_string();
        let actual = parse("hello '1 2 3\\'").to_string();
        assert_eq!(expected, actual);
    }
}
