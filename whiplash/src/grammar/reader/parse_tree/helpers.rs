pub struct Delim {
    pub count: i32,
    pub paren: String
}

impl Delim {
    pub fn new() -> Delim {
        Delim {
            count: 0,
            paren: "".to_string()
        }
    }
}

pub fn is_inv_paren(p1: String, p2: String) -> bool {
    (p1 == "(".to_string() && p2 == ")".to_string()) || 
    (p1 == "[".to_string() && p2 == "]".to_string()) 
}