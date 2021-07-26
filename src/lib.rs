mod parser;

pub use parser::Args;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_get_str() {
        let arg = Args::new("s*", vec![
            String::from("-s"), String::from("abc")]);
        assert!(arg.get_str("s").eq("abc"));
    }
    #[test]
    fn check_get_i32() {
        let arg = Args::new("i#", vec![
            String::from("-i"), String::from("1")]);
        assert_eq!(arg.get_i32("i"), 1);
    }
    #[test]
    fn check_get_bool() {
        let arg = Args::new("b", vec![
            String::from("-b")]);
        assert!(arg.get_bool("b"));
    }

    #[test]
    fn check_get_for_all_types() {
        let arg = Args::new("i#,s*,b", vec![
            String::from("-i"), String::from("1"), 
            String::from("-b"), 
            String::from("-s"), String::from("abc")]);
            assert_eq!(arg.get_i32("i"), 1);
            assert!(arg.get_bool("b"));
            assert!(arg.get_str("s").eq("abc"));
    }
}
