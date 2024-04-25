#[cfg(test)]
mod tests {
    use std::fmt::{Formatter, Write};
    use crate::{DisplayWithOptions, IndentingFormatter, IndentOptions, with_options};

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

    #[test]
    fn indent_twice() {
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

        let reference =
r#"A
    B
        C
    D
"#;

        assert_eq!(reference, tree_formatted.as_str());
    }
}
