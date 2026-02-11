/// Parsed input actions.
#[derive(Debug, PartialEq)]
pub enum InputAction<'a> {
    /// Empty line: run prefix only.
    Empty,
    /// :q / :exit
    Quit,
    /// :<shell command>
    ShellCommand(&'a str),
    /// +<command>
    Add(&'a str),
    /// - or -N
    Drop(usize),
    /// !<command>
    Replace(&'a str),
    /// Normal command executed with prefix.
    Execute(&'a str),
}

/// Parse input into an action.
pub fn parse_input(input: &str) -> InputAction<'_> {
    if input.is_empty() {
        return InputAction::Empty;
    }
    if input == ":q" || input == ":exit" {
        return InputAction::Quit;
    }
    if let Some(rest) = input.strip_prefix(':') {
        return InputAction::ShellCommand(rest.trim());
    }
    if let Some(rest) = input.strip_prefix('+') {
        return InputAction::Add(rest.trim());
    }
    if let Some(rest) = input.strip_prefix('!') {
        return InputAction::Replace(rest.trim());
    }
    if input == "-" {
        return InputAction::Drop(1);
    }
    if let Some(rest) = input.strip_prefix('-') {
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(count) = rest.parse::<usize>() {
                if count > 0 {
                    return InputAction::Drop(count);
                }
            }
        }
    }
    InputAction::Execute(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(parse_input(""), InputAction::Empty);
    }

    #[test]
    fn parse_quit_q() {
        assert_eq!(parse_input(":q"), InputAction::Quit);
    }

    #[test]
    fn parse_quit_exit() {
        assert_eq!(parse_input(":exit"), InputAction::Quit);
    }

    #[test]
    fn parse_shell_command() {
        assert_eq!(parse_input(":ls -la"), InputAction::ShellCommand("ls -la"));
    }

    #[test]
    fn parse_shell_command_trimmed() {
        assert_eq!(parse_input(": ls"), InputAction::ShellCommand("ls"));
    }

    #[test]
    fn parse_add() {
        assert_eq!(parse_input("+add"), InputAction::Add("add"));
    }

    #[test]
    fn parse_add_with_space() {
        assert_eq!(parse_input("+ add"), InputAction::Add("add"));
    }

    #[test]
    fn parse_drop_single() {
        assert_eq!(parse_input("-"), InputAction::Drop(1));
    }

    #[test]
    fn parse_drop_n() {
        assert_eq!(parse_input("-3"), InputAction::Drop(3));
    }

    #[test]
    fn parse_replace() {
        assert_eq!(parse_input("!commit"), InputAction::Replace("commit"));
    }

    #[test]
    fn parse_normal_command() {
        assert_eq!(parse_input("status"), InputAction::Execute("status"));
    }

    #[test]
    fn parse_normal_with_args() {
        assert_eq!(
            parse_input("log --oneline"),
            InputAction::Execute("log --oneline")
        );
    }
}
