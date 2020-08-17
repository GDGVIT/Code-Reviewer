pub struct InStringLit {
    pub in_string_literal: bool,
    pub terminal: Option<char>,
}

impl InStringLit {
    pub fn set(&mut self, in_string_literal: bool, terminal: Option<char>) {
        self.in_string_literal = in_string_literal;
        self.terminal = terminal;
    }

}