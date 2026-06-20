// define recognized commands and
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Noop,
    Help,
    About,
    Projects,
    Resume,
    Socials,
    Toggle,
    Clear,
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
}

pub const COMMANDS: &[CommandSpec] = &[
    CommandSpec {
        name: "/help",
        description: "Show available commands.",
    },
    CommandSpec {
        name: "/about",
        description: "Show a concise introduction.",
    },
    CommandSpec {
        name: "/projects",
        description: "Browse selected projects.",
    },
    CommandSpec {
        name: "/resume",
        description: "Open the resume link.",
    },
    CommandSpec {
        name: "/socials",
        description: "Browse social and profile links.",
    },
    CommandSpec {
        name: "/toggle",
        description: "Toggle light or dark theme.",
    },
    CommandSpec {
        name: "/clear",
        description: "Clear command output.",
    },
];

pub fn parse(input: &str) -> Command {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Command::Noop; // looks like empty input what does this return?
    }

    let token = trimmed.split_whitespace().next().unwrap_or_default();
    let normalized = if token.starts_with('/') {
        token.to_ascii_lowercase()
    } else {
        format!("/{}", token.to_ascii_lowercase())
    };

    match normalized.as_str() {
        "/help" => Command::Help,
        "/about" => Command::About,
        "/projects" => Command::Projects,
        "/resume" => Command::Resume,
        "/socials" => Command::Socials,
        "/toggle" => Command::Toggle,
        "/clear" => Command::Clear,
        _ => Command::Unknown(trimmed.to_string()),
    }
}

// test commands
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_commands() {
        assert_eq!(parse("/help"), Command::Help);
        assert_eq!(parse("/projects"), Command::Projects);
        assert_eq!(parse("/toggle"), Command::Toggle);
    }

    #[test]
    fn tolerates_missing_slash_and_whitespace() {
        assert_eq!(parse("  socials  "), Command::Socials);
        assert_eq!(parse("ABOUT now"), Command::About);
        assert_eq!(parse(""), Command::Noop);
    }

    #[test]
    fn preserves_unknown_command_text() {
        assert_eq!(
            parse("  /wat is this  "),
            Command::Unknown("/wat is this".to_string())
        );
    }
}
