// common library crate
pub mod config;
pub mod help;
pub mod model_content;
pub mod model_definition;
pub mod search;
pub mod tools;

pub const NR_OF_BOOKS_IN_TANACH: u16 = 4; // actual number of books is 39;

////////////////////////////////////////////////////////////
pub fn function_to_be_removed(text: &str) -> bool {
    println!("Common library called from crate: {}", text);
    true
}

#[cfg(test)]
mod ci_test {
    use crate::*;
    #[test]
    fn test_to_be_removed() {
        let true_false = function_to_be_removed("test_to_be_removed");
        assert!(true_false);
    }
}
