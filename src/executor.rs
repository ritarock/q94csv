use anyhow::{Ok, Result};

use crate::{command::Command, file::CSVReader};
pub struct Executor {
    command: Command,
    csv_reader: CSVReader,
}

#[derive(Debug, PartialEq)]
pub struct CliResult {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Executor {
    pub fn new(command: Command, file_path: String) -> Self {
        let csv_reader = CSVReader::new(file_path);
        Executor {
            command,
            csv_reader,
        }
    }

    pub fn run(&self) -> Result<CliResult> {
        match self.command {
            Command::All => self.all(),
            Command::Headers => self.headers(),
            Command::Count => self.count(),
        }
    }

    fn all(&self) -> Result<CliResult> {
        let read_csv = self.csv_reader.read()?;
        let header = read_csv[0].clone();
        let mut rows: Vec<Vec<String>> = Vec::new();

        for row in read_csv.iter().skip(1) {
            rows.push(row.clone());
        }

        Ok(CliResult { header, rows })
    }

    fn headers(&self) -> Result<CliResult> {
        let read_csv = self.csv_reader.read()?;
        let header = read_csv[0].clone();

        Ok(CliResult {
            header: header,
            rows: Vec::new(),
        })
    }

    fn count(&self) -> Result<CliResult> {
        let read_csv = self.csv_reader.read()?;

        Ok(CliResult {
            header: vec!["count".into()],
            rows: vec![
                vec![read_csv.iter().skip(1).len().to_string()]
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_succeed() -> anyhow::Result<()> {
        let header = vec!["id".into(), "team_id".into(), "name".into(), "note".into()];
        let rows = vec![
            vec!["1".into(), "1".into(), "name1".into(), "note1".into()],
            vec!["2".into(), "1".into(), "name2".into(), "note2".into()],
            vec!["3".into(), "2".into(), "name3".into(), "note3".into()],
            vec!["4".into(), "3".into(), "name4".into(), "note4".into()],
            vec!["5".into(), "4".into(), "name5".into(), "note5".into()],
            vec!["6".into(), "1".into(), "name6".into(), "note5".into()],
            vec!["7".into(), "2".into(), "name7".into(), "note6".into()],
        ];
        let want = CliResult { header, rows };

        let executor = Executor::new(Command::All, "sample.csv".into());
        let got = executor.all()?;
        assert_eq!(want, got);
        Ok(())
    }

    #[test]
    fn test_headers_succeed() -> anyhow::Result<()> {
        let header = vec!["id".into(), "team_id".into(), "name".into(), "note".into()];
        let want = CliResult {
            header,
            rows: Vec::new(),
        };

        let executor = Executor::new(Command::Headers, "sample.csv".into());
        let got = executor.headers()?;
        assert_eq!(want, got);
        Ok(())
    }

    #[test]
    fn test_count_succeed() -> anyhow::Result<()> {
        let header = vec!["count".into()];
        let rows = vec![vec!["7".into()]];
        let want = CliResult { header, rows };

        let executor = Executor::new(Command::Count, "sample.csv".into());
        let got = executor.count()?;
        assert_eq!(want, got);
        Ok(())
    }
}
