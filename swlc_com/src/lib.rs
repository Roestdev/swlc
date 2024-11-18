// common library crate
pub fn function_to_be_removed(text: &str) -> bool {
    println!("Common library called from crate: {}",text);
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
