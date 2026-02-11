mod input;
mod prefix;
mod prompt;

use clap::Parser;
use input::InputAction;
use prefix::Prefix;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

/// Command prefixing for continuous workflow
#[derive(Parser)]
#[command(name = "flow", version, about)]
struct Cli {
    /// Program and arguments to prefix
    #[arg(required = true)]
    prefix: Vec<String>,
}

struct FlowHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
    validator: MatchingBracketValidator,
}

impl FlowHelper {
    fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            validator: MatchingBracketValidator::new(),
        }
    }
}

impl Helper for FlowHelper {}

impl Completer for FlowHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for FlowHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for FlowHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

impl Validator for FlowHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        self.validator.validate(ctx)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if which::which(&cli.prefix[0]).is_err() {
        eprintln!("error: \"{}\" is not a valid executable", cli.prefix[0]);
        std::process::exit(1);
    }

    let mut prefix = Prefix::new(&cli.prefix);

    let mut rl: Editor<FlowHelper, DefaultHistory> = Editor::new()?;
    rl.set_helper(Some(FlowHelper::new()));

    let history_path = history_path();
    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    loop {
        let prompt_str = prompt::format_prompt(&prefix.display());
        match rl.readline(&prompt_str) {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match input::parse_input(&line) {
                    InputAction::Empty => {
                        execute_command(&prefix.build_command(""));
                    }
                    InputAction::Quit => break,
                    InputAction::ShellCommand(cmd) => {
                        execute_command(cmd);
                    }
                    InputAction::Add(cmd) => {
                        prefix.add(cmd);
                    }
                    InputAction::Drop(count) => {
                        if prefix.drop(count) {
                            break;
                        }
                    }
                    InputAction::Replace(cmd) => {
                        prefix.replace(cmd);
                    }
                    InputAction::Execute(cmd) => {
                        execute_command(&prefix.build_command(cmd));
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("error: {}", err);
                break;
            }
        }
    }

    if let Some(ref path) = history_path {
        let _ = rl.save_history(path);
    }

    Ok(())
}

fn history_path() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".flow_history"))
}

/// Run a command with sh -c.
fn execute_command(cmd: &str) {
    let status = std::process::Command::new("sh").arg("-c").arg(cmd).status();
    if let Err(err) = status {
        eprintln!("error: {}", err);
    }
}
