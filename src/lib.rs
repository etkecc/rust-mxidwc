use regex::Regex;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum PatternError {
    InvalidPattern(String),
    RegexError(regex::Error),
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            PatternError::RegexError(err) => write!(f, "Regex error: {}", err),
        }
    }
}

impl std::error::Error for PatternError {}

impl From<regex::Error> for PatternError {
    fn from(err: regex::Error) -> Self {
        PatternError::RegexError(err)
    }
}

pub type Result<T> = std::result::Result<T, PatternError>;

// match_user_id tells if the given user id is allowed according to the given list of regexes
pub fn match_user_id(user_id: &str, allowed: &[Regex]) -> bool {
    for regex in allowed {
        if regex.is_match(user_id) {
            return true;
        }
    }
    false
}

pub fn parse_patterns_vector(patterns: &[String]) -> Result<Vec<Regex>> {
    parse_patterns(&vec_string_to_str_slice(patterns))
}

// parse_patterns converts a list of wildcard patterns to a list of regular expressions
// See parse_pattern for details
pub fn parse_patterns(patterns: &[&str]) -> Result<Vec<Regex>> {
    let mut regexes = Vec::with_capacity(patterns.len());

    for &pattern in patterns {
        let regex = parse_pattern(pattern)?;
        regexes.push(regex);
    }

    Ok(regexes)
}

// parse_pattern parses a user wildcard pattern and returns a regular expression which corresponds to it
pub fn parse_pattern(pattern: &str) -> Result<Regex> {
    if !pattern.starts_with('@') {
        return Err(PatternError::InvalidPattern(
            "patterns need to be fully-qualified, starting with a @".to_string(),
        ));
    }

    let pattern = &pattern[1..];
    if pattern.contains('@') {
        return Err(PatternError::InvalidPattern(
            "patterns cannot contain more than one @".to_string(),
        ));
    }

    let parts: Vec<&str> = pattern.split(':').collect();
    if parts.len() != 2 {
        return Err(PatternError::InvalidPattern(
            "expected exactly 2 parts in the pattern, separated by `:`".to_string(),
        ));
    }

    let localpart = parts[0];
    let localpart_pattern = get_pattern(localpart)?;

    let domain = parts[1];
    let domain_pattern = get_pattern(domain)?;

    let pattern = format!("^@{}:{}$", localpart_pattern, domain_pattern);

    let regex = Regex::new(&pattern)?;

    Ok(regex)
}

fn get_pattern(part: &str) -> Result<String> {
    if part.is_empty() {
        return Err(PatternError::InvalidPattern(
            "rejecting empty part".to_string(),
        ));
    }

    let mut pattern = String::new();
    for c in part.chars() {
        if c == '*' {
            pattern.push_str("([^:@]*)");
        } else {
            pattern.push_str(&regex::escape(&c.to_string()));
        }
    }

    Ok(pattern)
}

fn vec_string_to_str_slice(vec: &[String]) -> Vec<&str> {
    vec.iter().map(|s| s.as_str()).collect()
}
