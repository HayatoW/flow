/// Command prefix manager
pub struct Prefix {
    parts: Vec<String>,
}

impl Prefix {
    /// Build an initial prefix from CLI args.
    pub fn new(args: &[String]) -> Self {
        Self {
            parts: args.to_vec(),
        }
    }

    /// Append words to the prefix.
    pub fn add(&mut self, input: &str) {
        if input.trim().is_empty() {
            return;
        }
        self.parts
            .extend(input.split_whitespace().map(|part| part.to_string()));
    }

    /// Drop the last n elements, clamped to at least one element.
    /// Returns true if the prefix only had one element (caller should exit).
    pub fn drop(&mut self, n: usize) -> bool {
        if self.parts.len() <= 1 {
            return true;
        }
        let keep = self.parts.len().saturating_sub(n).max(1);
        self.parts.truncate(keep);
        false
    }

    /// Replace the last element with new input (drop 1 then add).
    pub fn replace(&mut self, input: &str) {
        let _ = self.drop(1);
        self.add(input);
    }

    /// Build a full command string from prefix + input.
    pub fn build_command(&self, input: &str) -> String {
        if input.is_empty() {
            self.display()
        } else {
            format!("{} {}", self.display(), input)
        }
    }

    /// Display prefix in a single string.
    pub fn display(&self) -> String {
        self.parts.join(" ")
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_single() {
        let p = Prefix::new(&["git".into()]);
        assert_eq!(p.display(), "git");
    }

    #[test]
    fn new_compound() {
        let p = Prefix::new(&["java".into(), "Primes".into()]);
        assert_eq!(p.display(), "java Primes");
    }

    #[test]
    fn add_single() {
        let mut p = Prefix::new(&["git".into()]);
        p.add("add");
        assert_eq!(p.display(), "git add");
    }

    #[test]
    fn add_multiple_words() {
        let mut p = Prefix::new(&["git".into()]);
        p.add("log --oneline");
        assert_eq!(p.display(), "git log --oneline");
    }

    #[test]
    fn drop_one() {
        let mut p = Prefix::new(&["git".into(), "add".into()]);
        let should_exit = p.drop(1);
        assert!(!should_exit);
        assert_eq!(p.display(), "git");
    }

    #[test]
    fn drop_n() {
        let mut p = Prefix::new(&["git".into(), "add".into(), "-A".into()]);
        let should_exit = p.drop(2);
        assert!(!should_exit);
        assert_eq!(p.display(), "git");
    }

    #[test]
    fn drop_clamp_to_minimum() {
        let mut p = Prefix::new(&["git".into(), "add".into()]);
        let should_exit = p.drop(5);
        assert!(!should_exit);
        assert_eq!(p.display(), "git");
    }

    #[test]
    fn drop_last_returns_exit() {
        let mut p = Prefix::new(&["git".into()]);
        let should_exit = p.drop(1);
        assert!(should_exit);
    }

    #[test]
    fn replace_last() {
        let mut p = Prefix::new(&["git".into(), "add".into()]);
        p.replace("commit");
        assert_eq!(p.display(), "git commit");
    }

    #[test]
    fn build_command_with_input() {
        let p = Prefix::new(&["git".into()]);
        assert_eq!(p.build_command("status"), "git status");
    }

    #[test]
    fn build_command_empty_input() {
        let p = Prefix::new(&["git".into()]);
        assert_eq!(p.build_command(""), "git");
    }

    #[test]
    fn display_compound() {
        let p = Prefix::new(&["gcc".into(), "-o".into(), "output".into(), "input.c".into()]);
        assert_eq!(p.display(), "gcc -o output input.c");
    }
}
