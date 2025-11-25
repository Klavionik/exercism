use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Flags {
    show_line_num: bool,
    filename_only: bool,
    case_sensitive: bool,
    invert_match: bool,
    full_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut this = Self::default();

        for flag in flags {
            match *flag {
                "-n" => this.show_line_num = true,
                "-l" => this.filename_only = true,
                "-i" => this.case_sensitive = false,
                "-v" => this.invert_match = true,
                "-x" => this.full_match = true,
                &_ => {}
            }
        }

        this
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            show_line_num: false,
            filename_only: false,
            case_sensitive: true,
            invert_match: false,
            full_match: false,
        }
    }
}

struct Grep<'a> {
    pattern: &'a str,
    flags: &'a Flags,
    files: &'a [&'a str]
}

impl<'a> Grep<'a> {
    pub fn new(pattern: &'a str, flags: &'a Flags, files: &'a [&'a str]) -> Self {
        Self { pattern, flags, files }
    }

    pub fn matches(&self, pattern: &str, line: &str) -> bool {
        let matched;

        #[allow(clippy::collapsible_else_if)]
        if self.flags.full_match {
            if self.flags.case_sensitive {
                matched = pattern == line
            } else {
                matched = line.eq_ignore_ascii_case(pattern);
            }
        } else {
            if self.flags.case_sensitive {
                matched = line.contains(pattern);
            } else {
                matched = line.to_lowercase().contains(&pattern.to_lowercase());
            }
        }

        if self.flags.invert_match {
            !matched
        } else {
            matched
        }
    }

    pub fn format(&self, line: &str, filename: &str, line_num: usize) -> String {
        let mut new_line = String::new();

        if self.flags.filename_only {
            new_line.push_str(filename);
            return new_line;
        }

        if self.is_multifile() {
            new_line.push_str(&format!("{filename}:"));
        }

        if self.flags.show_line_num {
            new_line.push_str(&format!("{line_num}:"));
        }

        new_line.push_str(line);

        new_line
    }
    
    pub fn run(&mut self) -> Result<impl Iterator<Item = Result<String, Error>>, Error> {
        let mut curr_file = 0;
        let mut line_num = 0usize;
        let mut curr_reader = BufReader::new(File::open(self.files[curr_file])?);
        let mut next_file = false;
        let mut buffer = String::new();
        
        Ok(std::iter::from_fn(move || {
            loop {
                loop {
                    buffer.clear();
                    
                    if next_file {
                        next_file = false;
                        break
                    }
                    
                    let eof = curr_reader.read_line(&mut buffer).ok()? == 0;
                    
                    if eof {
                        line_num = 0;
                        break
                    }
                    
                    line_num += 1;
                    
                    let line = buffer.trim();

                    if self.matches(self.pattern, &line) {
                        let new_line = self.format(&line, &self.files[curr_file], line_num);
                        
                        if self.is_multifile() && self.flags.filename_only {
                            next_file = true
                        }
                        
                        return Some(Ok(new_line))
                    }
                }
                
                curr_file += 1;
                
                if curr_file >= self.files.len() {
                    return None
                }

                let file = File::open(self.files[curr_file]).ok()?;
                curr_reader = BufReader::new(file);
            }
        }))
    }
    
    fn is_multifile(&self) -> bool {
        self.files.len() > 1
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut lines = vec![];
    let mut grep = Grep::new(pattern, flags, files);
    
    for line in grep.run()? {
        lines.push(line?);
    }

    Ok(lines)
}