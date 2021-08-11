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
