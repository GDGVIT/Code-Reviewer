pub struct Delim {
    pub count: i32,
    pub paren: char
}

impl Delim {
    pub fn new() -> Delim {
        Delim {
            count: 0,
            paren: ' '
        }
    }
}