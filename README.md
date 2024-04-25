# display_with_options

This tiny (< 200 LOC) crate allows you to pass options around in Display and Debug.

It also adds a way to indent a formatter implicitly on fresh new lines.

Both components are technically separate, but interact in useful ways to provide a customizable
indentation and pretty printing pipeline. In contrast to similar crates, this one gives you full
control while maintaining a close relationship with rust core functionality.

## Usage Examples

### IndentingFormatter

```rust
use display_with_options::IndentingFormatter;

fn main() {
    let mut dst: Vec<u8> = vec![];
    
    writeln!(dst, "A").unwrap();  // A
    
    let mut f = IndentingFormatter::new(&mut dst, "\t");
    writeln!(f, "B").unwrap(); // \tB
}
```

### DisplayWithOptions

```rust
use std::fmt::{Formatter, Write};
use display_with_options::{DisplayWithOptions, IndentingFormatter, IndentOptions, with_options};

/// Tree-like structure
struct Node {
    name: String,
    children: Vec<Box<Node>>
}

impl<'a> DisplayWithOptions<IndentOptions<'a>> for Node {
    fn fmt(&self, f: &mut Formatter, options: &IndentOptions) -> std::fmt::Result {
        writeln!(f, "{}{}", options, self.name)?;

        let options = options.deeper();
        let mut f = IndentingFormatter::new(f, &options.full_indentation);
        let options = options.restart();

        for child in self.children.iter() {
            write!(f, "{}", with_options(child.as_ref(), &options))?;
        }

        Ok(())
    }
}

// Test the Code

fn main() {
    let tree = Node {
        name: "A".to_string(),
        children: vec![
            Box::new(Node {
                name: "B".to_string(),
                children: vec![Box::new(Node { name: "C".to_string(), children: vec![] })]
            }),
            Box::new(Node {
                name: "D".to_string(),
                children: vec![]
            }),
        ]
    };
    
     let tree_formatted = format!("{}", with_options(&tree, &IndentOptions {
         full_indentation: String::new(),
         next_level: "    ",
     }));
     // A\n
     //     B\n
     //         C\n
     //     D\n
}
```
