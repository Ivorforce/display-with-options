#[cfg(test)]
mod tests {
    use std::fmt::{Formatter, Write};
    use crate::{DisplayWithOptions, IndentingFormatter, IndentOptions, with_options};

    struct Node {
        name: String,
        children: Vec<Box<Node>>
    }

    impl Node {
        pub fn new(name: &str, children: Vec<Box<Node>>) -> Box<Node> {
            Box::new(Node {
                name: name.to_string(),
                children
            })
        }
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
        let tree = Node::new("A", vec![
            Node::new("B", vec![
                Node::new("C", vec![]),
            ]),
            Node::new("D", vec![]),
        ]);

        let options = IndentOptions::new("    ");
        let tree_formatted = format!("{}", with_options(tree.as_ref(), &options));

        let reference =
r#"A
    B
        C
    D
"#;

        assert_eq!(reference, tree_formatted.as_str());
    }
}
