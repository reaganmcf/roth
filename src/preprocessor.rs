use fancy_regex::Regex;
use miette::Result;
use std::{fs::File, io::Read};

static INCLUDE_REGEX: &str = r"^include\s*\((?<file>.+?(?=\)))\)";

pub struct PreProcessor {
    raw_source: String,
}

impl PreProcessor {
    pub fn new(buffer: &str) -> Self {
        Self {
            raw_source: buffer.to_string(),
        }
    }

    pub fn expand(mut self) -> Result<String> {
        self.expand_includes()
    }

    fn expand_includes(&mut self) -> Result<String> {
        let mut expanded_file = String::from(&self.raw_source);
        let re = Regex::new(INCLUDE_REGEX).unwrap();
        for cap in re.captures_iter(self.raw_source.as_str()) {
            let cap = cap.unwrap();
            let full_match = cap.get(0).unwrap().as_str();
            let raw_match = cap.name("file").unwrap().as_str();

            // trim first and last char since it matches with quotation marks
            let mut trimmed = String::from(raw_match);
            trimmed.pop();
            trimmed.remove(0);

            match File::open(&trimmed) {
                Ok(mut file) => {
                    let mut file_contents = String::new();
                    if file.read_to_string(&mut file_contents).is_err() {
                        todo!("can't read file at {}", trimmed);
                    }

                    expanded_file = expanded_file.replace(full_match, file_contents.as_str());
                }
                Err(_) => {
                    todo!("can't open file at {}", trimmed)
                }
            }
        }

        Ok(expanded_file)
    }
}
