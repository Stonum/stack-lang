pub struct Formatter {
    pub indent: usize,
    current: usize,
}

impl Formatter {
    pub fn new(indent: usize) -> Self {
        Self { indent, current: 0 }
    }

    pub fn tab(&self) -> String {
        " ".repeat(self.current)
    }

    pub fn indent_inc(&mut self) {
        self.current += self.indent;
    }

    pub fn indent_dec(&mut self) {
        self.current -= self.indent;
    }
}
