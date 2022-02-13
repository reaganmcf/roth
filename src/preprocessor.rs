use fancy_regex::Regex;
use miette::Result;
use std::{
    fs::File,
    io::Read,
    path::PathBuf,
};

use crate::error::ParseError;

static INCLUDE_REGEX: &str = r"include\s*\((?<file>.+?(?=\)))\)";

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
        let mut start = 0;
        while let Some(m) = re.captures_from_pos(&self.raw_source, start).unwrap() {
            let full_match = m.get(0).unwrap().as_str();
            let raw_file_match = m.get(1).unwrap().as_str();

            start = m.get(0).unwrap().start() + 1;

            // trim first and last char since it matches with quotation marks
            let mut trimmed = String::from(raw_file_match);
            trimmed.pop();
            trimmed.remove(0);

            let file_path = PathBuf::from(trimmed.clone());
            if !file_path.exists() {
                return Err(ParseError::CantIncludeNonExistentFile(
                    self.raw_source.clone(),
                    trimmed.clone(),
                    (start, full_match.len()).into(),
                )
                .into());
            }

            match File::open(&trimmed) {
                Ok(mut file) => {
                    let mut file_contents = String::new();
                    if file.read_to_string(&mut file_contents).is_err() {
                        return Err(ParseError::CantOpenOrReadIncludeFile(
                            self.raw_source.clone(),
                            trimmed.clone(),
                            (start, full_match.len()).into(),
                        )
                        .into());
                    }

                    expanded_file = expanded_file.replace(full_match, file_contents.as_str());
                }
                Err(_) => {
                    return Err(ParseError::CantOpenOrReadIncludeFile(
                        self.raw_source.clone(),
                        trimmed.clone(),
                        (start, full_match.len()).into(),
                    )
                    .into())
                }
            }
        }

        Ok(expanded_file)
    }
}
