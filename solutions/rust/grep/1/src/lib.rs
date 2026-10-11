use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
enum DisplayOptions {
    MatchingLine,
    FilesOnly,
    PrefixLineNumber,
}
#[derive(Debug)]
struct MatchOptions {
    invert: bool,
    case_sensitive: bool,
    whole_line: bool,
}
#[derive(Debug)]
pub struct Flags {
    display_options: DisplayOptions,
    match_options: MatchOptions,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            match_options: MatchOptions {
                invert: flags.contains(&"-v"),
                case_sensitive: !flags.contains(&"-i"),
                whole_line: flags.contains(&"-x"),
            },
            display_options: match (flags.contains(&"-l"), flags.contains(&"-n")) {
                (true, _) => DisplayOptions::FilesOnly,
                (false, true) => DisplayOptions::PrefixLineNumber,
                (false, false) => DisplayOptions::MatchingLine,
            },
        }
    }
}

struct Match {
    file: String,
    lineno: u16,
    match_text: String,
}
impl Match {
    fn new(
        pattern: &str,
        line: &str,
        file: &str,
        lineno: u16,
        match_options: &MatchOptions,
    ) -> Option<Self> {
        let mut pattern_copy = pattern.to_string();
        let mut line_copy = line.to_string();
        let file = file.to_string();
        if !match_options.case_sensitive {
            pattern_copy = pattern_copy.to_lowercase();
            line_copy = line_copy.to_lowercase();
        }
        let is_match = if match_options.whole_line {
            line_copy == pattern_copy
        } else {
            line_copy.contains(&pattern_copy)
        };
        let is_match = is_match ^ match_options.invert;
        is_match.then(|| Match {
            file,
            lineno,
            match_text: line.to_string(),
        })
    }
    fn format(&self, options: &DisplayOptions) -> String {
        match options {
            DisplayOptions::MatchingLine => self.match_text.clone(),
            DisplayOptions::FilesOnly => self.file.clone(),
            DisplayOptions::PrefixLineNumber => {
                format!("{}:{}", self.lineno, self.match_text)
            }
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut match_list = Vec::new();
    let mut search_file = |file| -> Result<(), Error> {
        let content = std::fs::read_to_string(file)?;
        for (i, line) in content.lines().enumerate() {
            let lineno = i as u16 + 1;
            if let Some(m) = Match::new(pattern, line, file, lineno, &flags.match_options) {
                if files.len() > 1 && !matches!(flags.display_options, DisplayOptions::FilesOnly) {
                    match_list.push(format!("{}:{}", file, m.format(&flags.display_options)));
                } else {
                    match_list.push(m.format(&flags.display_options));
                }
            }
        }
        Ok(())
    };
    for &file in files {
        search_file(file)?
    }
    match_list.dedup();
    Ok(match_list)
}
