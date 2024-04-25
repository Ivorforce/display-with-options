use std::fmt::Write;

pub struct IndentingFormatter<'a, 'b> {
    pub writer: &'a mut (dyn Write + 'a),
    pub is_fresh_line: bool,
    pub indentation: &'b str,
}

impl<'a, 'b> IndentingFormatter<'a, 'b> {
    pub fn new(writer: &'a mut (dyn Write + 'a), indentation: &'b str) -> IndentingFormatter<'a, 'b> {
        IndentingFormatter {
            writer,
            is_fresh_line: true,
            indentation,
        }
    }
}

impl<'a, 'b> Write for IndentingFormatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut split = s.split('\n');

        // First line: May append to a line, and has no preceding \n
        let first = split.next().unwrap();
        if !first.is_empty() {
            if self.is_fresh_line {
                self.writer.write_str(self.indentation)?;
            }

            self.writer.write_str(first)?;
            self.is_fresh_line = false;
        }

        // All lines after: start after fresh line
        for line in split {
            self.writer.write_char('\n')?;

            if line.is_empty() {
                self.is_fresh_line = true;
                continue;
            }

            self.writer.write_str(self.indentation)?;
            self.writer.write_str(line)?;
            self.is_fresh_line = false;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if c == '\n' {
            self.is_fresh_line = true;
        }
        else if self.is_fresh_line {
            self.writer.write_str(self.indentation)?;
            self.is_fresh_line = false;
        }

        self.writer.write_char(c)
    }
}
