use clap::Parser;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};
fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::parse();
    let r = engine.walk_dir()?;
    let mut stdout = io::stdout().lock();
    for (path, content) in r {
        engine.process(path, BufReader::new(content), &mut stdout)?;
    }
    Ok(())
}

struct LineInfo<'a> {
    path: &'a str,
    line: usize,
    data: String,
}

impl Display for LineInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}\n", self.path, self.line, self.data)
    }
}

const HELP_TEMPLATE: &'static str = "{bin} {version} ({author})

{about}
USAGE:
    {usage}
{all-args}
";

#[derive(Parser)]
#[command(version = "1.0", author = "cndoit18 <cndoit18@outlook.com>", help_template = HELP_TEMPLATE)]
struct Engine {
    query: String,
    glob: String,
}

impl Engine {
    fn process<T: Read, W: Write>(
        &self,
        path: String,
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
                        path: &path,
                        line: line + 1,
                        data: content
                    }
                )?;
            }
        }
        Ok(())
    }
    fn walk_dir(&self) -> Result<Vec<(String, Box<dyn Read>)>, Box<dyn Error>> {
        let mut r = Vec::<(String, Box<dyn Read>)>::new();
        for entry in glob::glob(&self.glob).expect("Failed to read glob pattern") {
            let path = entry?;
            if path.is_file() {
                r.push((path.display().to_string(), Box::new(File::open(path)?)));
            }
        }
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let engine = Engine {
            query: "hello".to_string(),
            glob: "ignore".to_string(),
        };
        let reader = BufReader::new("hello\nworld\nhello\n".as_bytes());
        let mut writer = Vec::new();
        engine.process("".to_string(), reader, &mut writer).unwrap();
        assert_eq!(writer, ":1: hello\n:3: hello\n".as_bytes());
    }

    #[test]
    fn test_walk_dir() {
        let engine = Engine {
            query: "hello".to_string(),
            glob: "**/main.rs".to_string(),
        };
        let r = engine.walk_dir().unwrap();
        assert_eq!(r.len(), 1);
    }
}
