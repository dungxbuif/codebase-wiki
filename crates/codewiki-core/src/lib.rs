//! Core command orchestration for CodeWiki.

use codewiki_detect::DetectionCapabilities;
use codewiki_docs::WikiDocsLayout;
use codewiki_store::StoreLayout;

/// Result of executing a CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOutput {
    /// Process exit code expected by the CLI entrypoint.
    pub exit_code: i32,
    /// Text written to stdout.
    pub stdout: String,
    /// Text written to stderr.
    pub stderr: String,
}

impl CliOutput {
    fn ok(stdout: impl Into<String>) -> Self {
        Self {
            exit_code: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }

    fn error(exit_code: i32, stderr: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: String::new(),
            stderr: stderr.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Help,
    Version,
    Status,
}

/// Parse and execute a CodeWiki command.
pub fn run<I, S>(args: I) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    match parse_command(args) {
        Ok(Command::Help) => CliOutput::ok(help_text()),
        Ok(Command::Version) => CliOutput::ok(format!("codewiki {}\n", env!("CARGO_PKG_VERSION"))),
        Ok(Command::Status) => CliOutput::ok(status_text()),
        Err(message) => CliOutput::error(2, message),
    }
}

fn parse_command<I, S>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(Command::Help);
    };

    if args.next().is_some() {
        return Err("error: expected a single command for this scaffold\n".to_string());
    }

    match first.as_ref() {
        "help" | "-h" | "--help" => Ok(Command::Help),
        "version" | "-V" | "--version" => Ok(Command::Version),
        "status" => Ok(Command::Status),
        unknown => Err(format!(
            "error: unknown command `{unknown}`\n\nRun `codewiki help` for available commands.\n"
        )),
    }
}

fn help_text() -> String {
    [
        "CodeWiki",
        "",
        "This binary is a companion tool for the CodeWiki skill.",
        "",
        "Usage:",
        "  codewiki help",
        "  codewiki version",
        "  codewiki status",
        "",
        "Planned companion commands:",
        "  codewiki doctor",
        "  codewiki inspect",
        "  codewiki cache",
        "",
    ]
    .join("\n")
}

fn status_text() -> String {
    let detection = DetectionCapabilities::scaffold();
    let store = StoreLayout::default();
    let docs = WikiDocsLayout::default();

    format!(
        "CodeWiki companion tool scaffold ready\nruntime: rust\ncommands: help, version, status\nplanned detection: {}\ncommitted config: {}\ncommitted plan: {}\nlocal agents: {}\nlocal state: {}\ndocs root: {}\n",
        detection.summary(),
        store.committed_config_path,
        store.committed_plan_path,
        store.committed_agents_path,
        store.local_state_summary,
        docs.generated_docs_root,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_prints_help() {
        let output = run(std::iter::empty::<&str>());

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("Usage:"));
    }

    #[test]
    fn version_prints_package_version() {
        let output = run(["version"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.starts_with("codewiki "));
    }

    #[test]
    fn status_mentions_rust_runtime() {
        let output = run(["status"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("runtime: rust"));
        assert!(output.stdout.contains(".codewiki/config.yml"));
        assert!(output.stdout.contains(".codewiki/AGENTS.md"));
    }

    #[test]
    fn unknown_command_fails() {
        let output = run(["wat"]);

        assert_eq!(output.exit_code, 2);
        assert!(output.stderr.contains("unknown command"));
    }
}
