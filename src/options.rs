use std::fmt::{Debug, Display, Formatter};
use crate::debug::DebugWithOptions;
use crate::display::DisplayWithOptions;

#[derive(Debug)]
pub struct IndentOptions<'a> {
    pub full_indentation: String,
    pub next_level: &'a str,
}

impl<'a> IndentOptions<'a> {
    pub fn deeper(&self) -> IndentOptions<'a> {
        IndentOptions {
            full_indentation: format!("{}{}", self.full_indentation, self.next_level),
            next_level: self.next_level,
        }
    }

    pub fn restart(&self) -> IndentOptions<'a> {
        IndentOptions {
            full_indentation: String::new(),
            next_level: self.next_level,
        }
    }
}

impl<'a> Display for IndentOptions<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_indentation)
    }
}

pub struct ObjectWithOptions<'a, 'b, Options, Object> {
    pub object: &'a Object,
    pub options: &'b Options,
}

impl<'a, 'b, Options, Object: DisplayWithOptions<Options>> Display for ObjectWithOptions<'a, 'b, Options, Object> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.object.fmt(f, self.options)
    }
}

impl<'a, 'b, Options, Object: DebugWithOptions<Options>> Debug for ObjectWithOptions<'a, 'b, Options, Object> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.object.fmt(f, self.options)
    }
}

pub fn with_options<'a, 'b, Options, Object: DisplayWithOptions<Options>>(object: &'a Object, options: &'b Options) -> ObjectWithOptions<'a, 'b, Options, Object> {
    ObjectWithOptions {
        object,
        options
    }
}
