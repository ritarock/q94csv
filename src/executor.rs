use anyhow::{Ok, Result};

use crate::{command::Command, file::CSVReader};
pub struct Executor {
    command: Command,
    csv_reader: CSVReader,
}

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
}
