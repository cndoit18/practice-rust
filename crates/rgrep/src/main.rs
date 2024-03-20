use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read, Write},
};

fn main() {
    println!("Hello, world!");
}

struct LineInfo {
    line: usize,
    data: String,
}

impl Display for LineInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", self.line, self.data)
    }
}

struct Engine {
    query: String,
}

impl Engine {
    fn process<T: Read, W: Write>(
        &self,
        reader: BufReader<T>,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        for (line, content) in reader.lines().enumerate() {
            let content = content?;
            if content.contains(&self.query) {
                write!(
                    writer,
                    "{}",
                    LineInfo {
                        line: line + 1,
                        data: content
                    }
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine() {
        let engine = Engine {
            query: "hello".to_string(),
        };
        let reader = BufReader::new("hello\nworld\nhello\n".as_bytes());
        let mut writer = Vec::new();
        engine.process(reader, &mut writer).unwrap();
        assert_eq!(writer, "1: hello\n3: hello\n".as_bytes());
    }
}
