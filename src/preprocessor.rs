use fancy_regex::Regex;
use miette::Result;
use std::{fs::File, io::Read, path::PathBuf};

use crate::error::ParseError;

static INCLUDE_REGEX: &str = r"include\s*\((?<file>.+?(?=\)))\)";
static MACRO_DEF_REGEX: &str = r"macro\s+(.+?)(?=\s)((.|\n|\r|\t)+?)(?=end)";

pub struct PreProcessor {
    source_raw: String,
    source_after_includes: String,
    source_after_macros: String
}

impl PreProcessor {
    pub fn new(buffer: &str) -> Self {
        Self {
            source_raw: buffer.to_string(),
            source_after_includes: String::new(),
            source_after_macros: String::new(),
        }
    }

    pub fn expand(mut self) -> Result<String> {
        self.expand_includes()?;
        self.expand_macros()?;

        Ok(self.source_after_macros)
    }

    fn expand_macros(&mut self) -> Result<()> {
        let mut expanded_file = self.source_after_includes.clone();
    
        let re = Regex::new(MACRO_DEF_REGEX).unwrap();
        let mut start = 0;
        while let Some(m) = re.captures_from_pos(&self.source_after_includes, start).unwrap() {
            let full_match = m.get(0).unwrap().as_str();
            let macro_name = m.get(1).unwrap().as_str();
            let macro_body = m.get(2).unwrap().as_str();

            //println!("Found match: {:?} - name = '{}', body = '{}'", full_match, macro_name, macro_body);

            start = m.get(0).unwrap().start() + full_match.len() + 3; // adding 3 for the 'end' keyword

            // replace all occurrences AFTER THE START POSITION in the file with the macro body
            let (front, rest) = expanded_file.split_at(start);
            //println!("Front part: {}", front);
            //println!("Rest part: {}", rest);

            let expanded_rest = rest.replace(macro_name, macro_body);

            expanded_file = format!("{}{}", front, expanded_rest);
        }

        self.source_after_macros = expanded_file;
        Ok(())
    }

    fn expand_includes(&mut self) -> Result<()> {
        let mut expanded_file = String::from(&self.source_raw);
        let re = Regex::new(INCLUDE_REGEX).unwrap();
        let mut start = 0;
        while let Some(m) = re.captures_from_pos(&self.source_raw, start).unwrap() {
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
                    self.source_raw.clone(),
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
                            self.source_raw.clone(),
                            trimmed.clone(),
                            (start, full_match.len()).into(),
                        )
                        .into());
                    }

                    expanded_file = expanded_file.replace(full_match, file_contents.as_str());
                }
                Err(_) => {
                    return Err(ParseError::CantOpenOrReadIncludeFile(
                        self.source_raw.clone(),
                        trimmed.clone(),
                        (start, full_match.len()).into(),
                    )
                    .into())
                }
            }
        }

        self.source_after_includes = expanded_file;
        Ok(())
    }
}
