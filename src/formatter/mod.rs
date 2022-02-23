use pad::{Alignment, PadStr};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Formatter {
    pub pad: usize,
    pub depth: usize,
    pub offset: usize,
    buffer: String,
}

impl Formatter {
    fn new() -> Formatter {
        Formatter {
            pad: 0,
            offset: 0,
            depth: 0,
            buffer: String::from(""),
        }
    }

    pub fn set_offset(&mut self, x: usize) -> &mut Self {
        self.offset = x;
        self
    }

    pub fn pop_context(&mut self) -> &mut Self {
        self.depth -= 1;
        self
    }

    pub fn new_context(&mut self) -> &mut Self {
        self.depth += 1;
        self
    }

    pub fn set_pad(&mut self, x: usize) -> &mut Self {
        self.pad = x;
        self
    }

    pub fn output(&self) -> String {
        String::from(&self.buffer)
    }

    pub fn space(&mut self) -> &mut Self {
        self.buffer = format!("{} ", self.buffer);
        self
    }

    pub fn new_line(&mut self) -> &mut Self {
        self.buffer = format!("{}\n", self.buffer);
        self
    }

    pub fn append(&mut self, s: &dyn fmt::Display) -> &mut Self {
        self.buffer = format!("{}{s}", self.buffer);
        self
    }

    pub fn ws(&mut self) -> &mut Self {
        self.buffer = format!("{} ", self.buffer);
        self
    }

    pub fn append_str(&mut self, s: &str) -> &mut Self {
        self.buffer = format!("{}{s}", self.buffer);
        self
    }

    pub fn test(&mut self, s: &dyn fmt::Display) -> &mut Self {
        self.buffer = format!(
            "{}{}{}",
            self.buffer,
            &"".pad_to_width_with_alignment(self.depth * self.pad, Alignment::Right),
            s.to_string()
                .pad_to_width_with_alignment(self.pad, Alignment::Right),
        );
        self
    }

    pub fn append_with_padding(&mut self, s: &dyn fmt::Display) -> &mut Self {
        self.test(s)
        /*self.buffer = format!(
            "{}{}",
            self.buffer,
            s.to_string()
                .pad_to_width_with_alignment(self.pad, Alignment::Right),
        );
        self*/
    }

    pub fn format(&self, left: String, right: String) -> String {
        format!(
            "{} {}",
            left.pad_to_width_with_alignment(self.pad, Alignment::Right),
            right
        )
    }

    pub fn pad_string(&self, left: String) -> String {
        left.pad_to_width_with_alignment(self.pad, Alignment::Right)
    }

    pub fn append_format(&mut self, s: &dyn Format) -> &mut Self {
        s.format(self)
    }

    pub fn indent(&mut self, s: &dyn Format) -> &mut Self {
        self.append(&"    ");
        s.format(self)
    }

    pub fn append_left_right(&mut self, left: &dyn fmt::Display, right: &dyn Format) -> &mut Self {
        self.left_side(left).space().append_format(right)
    }

    pub fn append_clause(&mut self, s: &dyn Format) -> &mut Self {
        self.new_line();
        s.format(self)
    }

    pub fn left_side(&mut self, s: &dyn fmt::Display) -> &mut Self {
        self.append_str(&s.to_string().pad_to_width_with_alignment(
            (self.depth * self.pad) + self.pad + self.offset,
            Alignment::Right,
        ));
        self
    }

    pub fn right_side(&mut self, s: &dyn Format) -> &mut Self {
        self.buffer = format!(
            "{}{}",
            self.buffer,
            "".pad_to_width_with_alignment(
                self.pad + (self.depth * self.pad) + 1 + self.offset,
                Alignment::Right
            ),
        );
        self.append_format(s)
    }
}

pub trait Format {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter;

    fn side(&self) -> usize {
        0
    }

    fn output(&self) -> String {
        let mut f = Formatter::new();
        self.format(&mut f).output()
    }
}

impl<T: std::fmt::Display> Format for T {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self)
    }
}

#[cfg(test)]
mod tests;
