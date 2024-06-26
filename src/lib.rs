//! # display-with-options
//!
//! This tiny (< 200 LOC) crate allows you to pass options around in Display and Debug.
//!
//! It also adds a way to indent a formatter implicitly on fresh new lines.
//!
//! Both components are technically separate, but interact in useful ways to provide a customizable
//! indentation and pretty printing pipeline. In contrast to similar crates, this one gives you full
//! control while maintaining a close relationship with rust core functionality.
//!
//!
//! ## Usage Examples
//!
//! ### IndentingFormatter
//!
//! ```rust
//! use display_with_options::IndentingFormatter;
//!
//! fn main() {
//!     let mut dst: Vec<u8> = vec![];
//!
//!     writeln!(dst, "A").unwrap();
//!
//!     let mut f = IndentingFormatter::new(&mut dst, "    ");
//!     writeln!(f, "B").unwrap();
//! }
//! ```
//!
//! Result:
//! ```text
//! A
//!     B
//! ```
//!
//! ### DisplayWithOptions
//!
//! ```rust
//! use std::fmt::{Formatter, Write};
//! use display_with_options::{DisplayWithOptions, IndentingFormatter, IndentOptions, with_options};
//!
//! /// Tree-like structure
//! struct Node {
//!     name: String,
//!     children: Vec<Box<Node>>
//! }
//!
//! impl Node {
//!     pub fn new(name: &str, children: Vec<Box<Node>>) -> Box<Node> {
//!         Box::new(Node {
//!             name: name.to_string(),
//!             children
//!         })
//!     }
//! }
//!
//! impl<'a> DisplayWithOptions<IndentOptions<'a>> for Node {
//!     fn fmt(&self, f: &mut Formatter, options: &IndentOptions) -> std::fmt::Result {
//!         writeln!(f, "{}{}", options, self.name)?;
//!
//!         let options = options.deeper();
//!         let mut f = IndentingFormatter::new(f, &options.full_indentation);
//!         let options = options.restart();
//!
//!         for child in self.children.iter() {
//!             write!(f, "{}", with_options(child.as_ref(), &options))?;
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! // Test the Code
//!
//! fn main() {
//!     let tree = Node::new("A", vec![
//!         Node::new("B", vec![
//!             Node::new("C", vec![]),
//!         ]),
//!         Node::new("D", vec![]),
//!     ]);
//!
//!     let options = IndentOptions::new("    ");
//!     println!("{}", with_options(tree.as_ref(), &options));
//! }
//! ```
//!
//! Result:
//! ```text
//! A
//!     B
//!         C
//!     D
//! ```
//!


mod debug;
mod display;
mod indenting_formatter;
mod options;
mod tests;

pub use debug::DebugWithOptions;
pub use display::DisplayWithOptions;
pub use indenting_formatter::IndentingFormatter;
pub use options::{with_options, IndentOptions};
