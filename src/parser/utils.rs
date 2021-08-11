pub fn is_some<T>(opt: Option<T>) -> bool{
    match opt {
        Some(_) => true,
        None => false,
    }
}

pub fn cut_last_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next_back();
    chars.as_str()
}

pub fn cut_first_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

pub fn get_arg_value(args: Vec<String>, key: &str) -> String {
    match args.iter().position(|n| format!("-{}", key).eq(n)) {
        Some(v) => args[v+1].to_string(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_cut_last_char() { 
        let val = "abcd";
        assert_eq!(
            cut_last_char(val),
            "abc"
        );
    }
    #[test]
    fn try_cut_first_char() { 
        let val = "abcd";
        assert_eq!(
            cut_first_char(val),
            "bcd"
        );
    }
    #[test]
    fn check_is_some_option() {
        let opt = Some(0);
        assert!(is_some(opt));
    }
    #[test]
    fn check_is_some_none() {
        let opt = None::<i32>;
        assert!(!is_some(opt));
    }
    #[test]
    fn try_get_arg_value_which_exist(){
        let vec = vec![
            "-a".to_string(), "0".to_string(), 
            "-b".to_string(), "1".to_string(), 
            "-c".to_string()
        ];
        assert_eq!(get_arg_value(vec.clone(), "a").as_str(), "0");
        assert_eq!(get_arg_value(vec.clone(), "b").as_str(), "1");
    }
    #[test]
    fn try_get_arg_value_which_does_not_exist(){
        let vec = vec![
            "-a".to_string(), "0".to_string(), 
            "-b".to_string(), "1".to_string(), 
            "-c".to_string()
        ];
        assert_eq!(get_arg_value(vec.clone(), "a").as_str(), "0");
        assert_eq!(get_arg_value(vec.clone(), "b").as_str(), "1");
        assert_eq!(get_arg_value(vec.clone(), "c"), "");
    }
}
