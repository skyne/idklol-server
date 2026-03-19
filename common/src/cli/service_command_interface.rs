use std::io::Write;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceControlCommand {
    Help,
    Restart,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceCommandEvent {
    Control(ServiceControlCommand),
    Custom { name: String, args: String },
}

#[derive(Debug, Clone, Copy)]
pub struct CustomCommandSpec {
    pub name: &'static str,
    pub usage: &'static str,
    pub description: &'static str,
}

pub fn parse_service_command_line(line: &str) -> Option<ServiceCommandEvent> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    if !trimmed.starts_with('.') {
        return Some(ServiceCommandEvent::Custom {
            name: "chat".to_string(),
            args: trimmed.to_string(),
        });
    }

    let without_prefix = &trimmed[1..];
    let mut parts = without_prefix.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim().to_ascii_lowercase();
    let args = parts.next().unwrap_or("").trim().to_string();

    let command = match name.as_str() {
        "help" | "h" => ServiceCommandEvent::Control(ServiceControlCommand::Help),
        "rs" | "restart" => ServiceCommandEvent::Control(ServiceControlCommand::Restart),
        "shutdown" | "quit" | "exit" => {
            ServiceCommandEvent::Control(ServiceControlCommand::Shutdown)
        }
        _ => ServiceCommandEvent::Custom { name, args },
    };

    Some(command)
}

pub fn build_help_text(service_name: &str, custom_commands: &[CustomCommandSpec]) -> String {
    let mut lines = vec![
        format!("{service_name} command interface"),
        "Built-in commands:".to_string(),
        "  .help                    Show this help".to_string(),
        "  .rs                      Graceful service restart".to_string(),
        "  .shutdown                Graceful service shutdown".to_string(),
    ];

    if !custom_commands.is_empty() {
        lines.push("Custom commands:".to_string());
        for custom in custom_commands {
            lines.push(format!(
                "  .{} {}",
                custom.name,
                if custom.usage.is_empty() {
                    "".to_string()
                } else {
                    custom.usage.to_string()
                }
            )
            .trim_end()
            .to_string());
            lines.push(format!("      {}", custom.description));
        }
    }

    lines.push("Tip: plain text without a dot is treated as chat input.".to_string());
    lines.join("\n")
}

pub async fn run_stdin_command_loop(
    command_tx: UnboundedSender<ServiceCommandEvent>,
    service_name: &'static str,
    custom_commands: &'static [CustomCommandSpec],
) -> Result<(), std::io::Error> {
    println!("{}", build_help_text(service_name, custom_commands));

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        print!("> ");
        let _ = std::io::stdout().flush();

        let Some(line) = reader.next_line().await? else {
            break;
        };

        if let Some(command) = parse_service_command_line(&line) {
            if command_tx.send(command).is_err() {
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        parse_service_command_line, ServiceCommandEvent, ServiceControlCommand,
    };

    #[test]
    fn parses_help_command() {
        let parsed = parse_service_command_line(".help").expect("command should parse");
        assert_eq!(parsed, ServiceCommandEvent::Control(ServiceControlCommand::Help));
    }

    #[test]
    fn parses_restart_alias() {
        let parsed = parse_service_command_line(".rs").expect("command should parse");
        assert_eq!(parsed, ServiceCommandEvent::Control(ServiceControlCommand::Restart));
    }

    #[test]
    fn parses_shutdown_alias() {
        let parsed = parse_service_command_line(".exit").expect("command should parse");
        assert_eq!(
            parsed,
            ServiceCommandEvent::Control(ServiceControlCommand::Shutdown)
        );
    }

    #[test]
    fn parses_custom_chat_with_args() {
        let parsed = parse_service_command_line(".chat hello there")
            .expect("command should parse");

        assert_eq!(
            parsed,
            ServiceCommandEvent::Custom {
                name: "chat".to_string(),
                args: "hello there".to_string(),
            }
        );
    }

    #[test]
    fn maps_plain_text_to_chat_command() {
        let parsed = parse_service_command_line("hello npc")
            .expect("plain text should be mapped to chat");

        assert_eq!(
            parsed,
            ServiceCommandEvent::Custom {
                name: "chat".to_string(),
                args: "hello npc".to_string(),
            }
        );
    }
}
