use std::collections::HashMap;

pub struct Args {
    strings: HashMap<String, String>,
    bools:   HashMap<String, bool>,
    ints:    HashMap<String, i32>,
}

impl Args {
    pub fn new(schema: &str, args: Vec<String>) -> Args{
        let mut strings: HashMap<String, String> = HashMap::new();
        let mut bools: HashMap<String, bool>   = HashMap::new();
        let mut ints: HashMap<String, i32>     = HashMap::new();
        
        let split = schema.split(",");

        for s in split {
            match s.chars().last().unwrap() {
                '#' => {
                    let mut chars = s.chars();
                    chars.next_back();
                    ints.insert(chars.as_str().to_string(), 0);
                },
                '*' => {
                    let mut chars = s.chars();
                    chars.next_back();
                    strings.insert(chars.as_str().to_string(), String::new());
                },
                _ => { bools.insert(s.to_string(), false); },
            }
        }
        
        for key in ints.clone().keys(){
            match args.iter().position(|n| format!("-{}", key).eq(n)) {
                Some(v) => { ints.insert(key.to_string(), args[v+1].parse::<i32>().unwrap()); },
                None => {},
            }
        }

        for key in bools.clone().keys(){
            match args.iter().position(|n| format!("-{}", key).eq(n)) {
                Some(_) => { bools.insert(key.to_string(), true); },
                None => {},
            }
        }

        for key in strings.clone().keys(){
            match args.iter().position(|n| format!("-{}", key).eq(n)) {
                Some(v) => { strings.insert(key.to_string(), args[v+1].clone()); },
                None => {},
            }
        }

        Args {strings: strings, bools: bools, ints: ints}
    }

    pub fn get_i32(&self, name: &str) -> i32{
        match &self.ints.get(name) {
            Some(v) => **v,
            None => 0,
        }
    }
    pub fn get_str(&self, name: &str) -> &str{
        match &self.strings.get(name) {
            Some(v) => v.as_str(),
            None => "",
        }
    }
    pub fn get_bool(&self, name: &str) -> bool{
        match &self.bools.get(name) {
            Some(v) => **v,
            None => false,
        }
    }
}
