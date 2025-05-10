use anyhow::{Context, Ok, Result, anyhow};
use std::io;
use std::io::Write;

use crate::command::Command;
use crate::executor::CliResult;

#[derive(Debug)]
pub struct Handler {
    args: Vec<String>,
}

impl Handler {
    pub fn new(args: Vec<String>) -> Handler {
        Handler { args }
    }

    pub fn run(&self) -> Result<(Command, String)> {
        self.validate()?;

        let file_path = self.get_file_path()?;
        let cmd = self.get_command()?;

        Ok((cmd, file_path.clone()))
    }

    fn validate(&self) -> Result<()> {
        self.args.get(0).with_context(|| "not enough args")?;
        let cmd = self.args.get(1).with_context(|| "no command provided")?;

        if self.args.len() > 2 {
            return Err(anyhow!("too many args"));
        }

        match cmd.as_str() {
            "all" => Ok(()),
            "headers" => Ok(()),
            "count" => Ok(()),
            _ => Err(anyhow!("unkown command: {}", cmd)),
        }
    }

    fn get_command(&self) -> Result<Command> {
        let cmd = self.args.get(1).with_context(|| "not enough args")?;

        match cmd.as_str() {
            "all" => Ok(Command::All),
            "headers" => Ok(Command::Headers),
            "count" => Ok(Command::Count),
            _ => Err(anyhow!("unkown command: {}", cmd)),
        }
    }

    fn get_file_path(&self) -> Result<String> {
        let file = self.args.get(0).with_context(|| "not enough args")?;
        Ok(file.clone())
    }

    fn is_command_headers(&self) -> bool {
        let cmd = self.get_command().unwrap_or(Command::Headers);
        match cmd {
            Command::Headers => true,
            _ => false,
        }
    }

    pub fn display(&self, cli_result: CliResult) {
        let mut output = io::stdout();

        if self.is_command_headers() {
            let _ = writeln!(output, "{}", cli_result.header.join(", "));
            return;
        }

        let _ = writeln!(output, "{}", cli_result.header.join("\t"));

        let separators: Vec<String> = cli_result
            .header
            .iter()
            .map(|h| "-".repeat(h.len()))
            .collect();
        let _ = writeln!(output, "{}", separators.join("\t"));

        for row in &cli_result.rows {
            let _ = writeln!(output, "{}", row.join("\t"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_succeed() -> anyhow::Result<()> {
        let handler = Handler::new(vec!["test.csv".into(), "all".into()]);
        assert!(handler.validate().is_ok());

        let handler = Handler::new(vec!["test.csv".into(), "headers".into()]);
        assert!(handler.validate().is_ok());

        let handler = Handler::new(vec!["test.csv".into(), "count".into()]);
        assert!(handler.validate().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_failed_not_enough_args() {
        let handler = Handler::new(vec![]);
        let err = handler.validate().unwrap_err();
        assert_eq!(err.to_string(), "not enough args")
    }

    #[test]
    fn test_validate_failed_no_command_provided() {
        let handler = Handler::new(vec!["test.csv".into()]);
        let err = handler.validate().unwrap_err();
        assert_eq!(err.to_string(), "no command provided");
    }

    #[test]
    fn test_validate_failed_too_many_args() {
        let handler = Handler::new(vec!["test.csv".into(), "all".into(), "extra".into()]);
        let err = handler.validate().unwrap_err();
        assert_eq!(err.to_string(), "too many args");
    }

    #[test]
    fn test_validate_failed_unkown_command() {
        let handler = Handler::new(vec!["test.csv".into(), "delete".into()]);
        let err = handler.validate().unwrap_err();
        assert_eq!(err.to_string(), "unkown command: delete");
    }

    #[test]
    fn test_get_command_succeed() -> anyhow::Result<()> {
        let handler = Handler::new(vec!["test.csv".into(), "all".into()]);
        let command = handler.get_command()?;
        assert_eq!(command, Command::All);

        let handler = Handler::new(vec!["test.csv".into(), "headers".into()]);
        let command = handler.get_command()?;
        assert_eq!(command, Command::Headers);

        let handler = Handler::new(vec!["test.csv".into(), "count".into()]);
        let command = handler.get_command()?;
        assert_eq!(command, Command::Count);
        Ok(())
    }

    #[test]
    fn test_get_command_failed_not_enough_args() {
        let handler = Handler::new(vec!["test.csv".into()]);
        let err = handler.get_command().unwrap_err();
        assert_eq!(err.to_string(), "not enough args");
    }

    #[test]
    fn test_get_command_failed_unkown_command() {
        let handler = Handler::new(vec!["test.csv".into(), "delete".into()]);
        let err = handler.get_command().unwrap_err();
        assert_eq!(err.to_string(), "unkown command: delete");
    }

    #[test]
    fn test_get_file_path_succeed() -> anyhow::Result<()> {
        let handler = Handler::new(vec!["test.csv".into(), "delete".into()]);
        let file_path = handler.get_file_path()?;
        assert_eq!(file_path, "test.csv");
        Ok(())
    }

    #[test]
    fn test_get_file_path_failed_not_enough_args() {
        let handler = Handler::new(vec![]);
        let err = handler.get_file_path().unwrap_err();
        assert_eq!(err.to_string(), "not enough args");
    }
}
