use std::fmt::{Debug, Display, Formatter};
use crate::debug::DebugWithOptions;
use crate::display::DisplayWithOptions;

/// Indentation state. Holds what was previously intended,
///  and what is intended for further indention.
#[derive(Debug)]
pub struct IndentOptions<'a> {
    pub full_indentation: String,
    pub next_level: &'a str,
}

impl<'a> IndentOptions<'a> {
    pub fn new(indent: &'a str) -> Self {
        IndentOptions {
            full_indentation: "".to_string(),
            next_level: indent,
        }
    }

    /// Indents one more level with the same indentation as before.
    pub fn deeper(&self) -> IndentOptions<'a> {
        IndentOptions {
            full_indentation: format!("{}{}", self.full_indentation, self.next_level),
            next_level: self.next_level,
        }
    }

    /// Indents multiple levels with the same indentation as before.
    pub fn n_deeper(&self, indents: usize) -> IndentOptions<'a> {
        IndentOptions {
            full_indentation: format!("{}{}", self.full_indentation, self.next_level.repeat(indents)),
            next_level: self.next_level,
        }
    }

    /// Switches the indentation style, but keeps the previous indentation.
    pub fn with_next<'b>(&self, indent: &'b str) -> IndentOptions<'b> {
        IndentOptions {
            full_indentation: self.full_indentation.clone(),
            next_level: indent,
        }
    }

    /// Appends the given indentation once, but keeps the previous style for further levels.
    pub fn appending<'b>(&self, indent: &str) -> IndentOptions<'a> {
        IndentOptions {
            full_indentation: format!("{}{}", self.full_indentation, indent),
            next_level: self.next_level,
        }
    }

    /// Resets the indentation, but keeps the indentation style.
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

/// Wraps the object with its options such that it can be written with Display or Debug.
/// To work in format strings, the object must implement `DisplayWithOptions` or `DebugWithOptions`.
pub fn with_options<'a, 'b, Options, Object>(object: &'a Object, options: &'b Options) -> ObjectWithOptions<'a, 'b, Options, Object> {
    ObjectWithOptions {
        object,
        options
    }
}
