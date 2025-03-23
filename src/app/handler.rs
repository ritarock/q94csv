use std::io;
use std::io::Write;

use crate::{entity::query::Query, infra::file::CSVReader};

use super::executor::{Executor, Results};

pub struct Handler {
    executor: Executor,
}

impl Handler {
    pub fn new() -> Self {
        let csv_reader = CSVReader::new();
        let executor = Executor::new(csv_reader);
        Handler { executor }
    }

    pub fn run(&self, args: Vec<String>) -> Result<(), String> {
        self.validate(&args).map_err(|err| err)?;

        let query = Query::new(&args[1]);
        query.validate().map_err(|err| err)?;

        let result = self.executor.execute(query).map_err(|err| err)?;
        self.display_results(result);
        Ok(())
    }

    fn validate(&self, args: &Vec<String>) -> Result<(), String> {
        if args.len() < 2 {
            return Err("not enough args".to_string());
        }

        if args[1] == "" {
            return Err("not enough args".to_string());
        }

        if args.len() >= 3 {
            return Err("too many args".to_string());
        }

        Ok(())
    }

    fn display_results(&self, result: Results) {
        let mut output = io::stdout();

        let _ = writeln!(output, "{}", result.header.join("\t"));

        let separators: Vec<String> = result.header.iter().map(|h| "-".repeat(h.len())).collect();
        let _ = writeln!(output, "{}", separators.join("\t"));

        for row in &result.rows {
            let _ = writeln!(output, "{}", row.join("\t"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_validate() {
        let handler = Handler::new();

        assert!(
            handler
                .validate(&vec!["system_arg".to_string(), "target_arg".to_string()])
                .is_ok()
        );

        assert_eq!(
            handler.validate(&vec!["system_arg".to_string()]),
            Err("not enough args".to_string())
        );
        assert_eq!(
            handler.validate(&vec![
                "system_arg".to_string(),
                "target_arg".to_string(),
                "extra".to_string()
            ]),
            Err("too many args".to_string())
        );
    }
}
