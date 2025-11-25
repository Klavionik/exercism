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

struct GrepMatcher<'a> {
    flags: &'a Flags,
}

impl<'a> GrepMatcher<'a> {
    pub fn new(flags: &'a Flags) -> Self {
        Self { flags }
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
}

struct GrepFormatter<'a> {
    is_multifile: bool,
    flags: &'a Flags,
}

impl<'a> GrepFormatter<'a> {
    pub fn new(flags: &'a Flags, is_multifile: bool) -> Self {
        Self {
            flags,
            is_multifile,
        }
    }

    pub fn format(&self, line: &str, filename: &str, line_idx: usize) -> String {
        let mut new_line = String::new();

        if self.flags.filename_only {
            new_line.push_str(filename);
            return new_line;
        }

        if self.is_multifile {
            new_line.push_str(&format!("{filename}:"));
        }

        if self.flags.show_line_num {
            let line_num = line_idx + 1;
            new_line.push_str(&format!("{line_num}:"));
        }

        new_line.push_str(line);

        new_line
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut lines = vec![];
    let is_multifile = files.len() > 1;
    let formatter = GrepFormatter::new(flags, is_multifile);
    let matcher = GrepMatcher::new(flags);

    for filename in files {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for (line_idx, line) in reader.lines().enumerate() {
            let line = line?;

            if matcher.matches(pattern, &line) {
                let new_line = formatter.format(&line, filename, line_idx);
                lines.push(new_line);

                if is_multifile && flags.filename_only {
                    break
                }
            }
        }
    }

    Ok(lines)
}