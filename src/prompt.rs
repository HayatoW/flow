const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const RESET: &str = "\x1b[0m";
const MARKER_START: &str = "\x01";
const MARKER_END: &str = "\x02";

fn paint(text: &str, color: &str) -> String {
    format!(
        "{start}{color}{end}{text}{start}{reset}{end}",
        start = MARKER_START,
        end = MARKER_END,
        color = color,
        text = text,
        reset = RESET
    )
}

/// Format the prompt string for rustyline (with invisible markers).
pub fn format_prompt(prefix_display: &str) -> String {
    format!(
        "{} {}{} ",
        paint("$", YELLOW),
        paint(prefix_display, CYAN),
        paint(">", WHITE)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_prompt_single() {
        let result = format_prompt("git");
        let stripped: String = result
            .chars()
            .filter(|c| !matches!(c, '\x01' | '\x02'))
            .collect();
        assert!(stripped.contains("git"));
        assert!(stripped.contains("$"));
        assert!(stripped.contains(">"));
    }

    #[test]
    fn format_prompt_compound() {
        let result = format_prompt("git add");
        let stripped: String = result
            .chars()
            .filter(|c| !matches!(c, '\x01' | '\x02'))
            .collect();
        assert!(stripped.contains("git add"));
    }
}
