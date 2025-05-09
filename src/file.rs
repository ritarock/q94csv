use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

pub struct CSVReader {
    file_path: String,
}

impl CSVReader {
    pub fn new(file_path: String) -> Self {
        CSVReader { file_path }
    }

    pub fn read(&self) -> Result<Vec<Vec<String>>> {
        let file = File::open(&self.file_path)
            .with_context(|| format!("failed to open: {}", self.file_path))?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            let line = line.context("error occurred while loading the file")?;
            let row: Vec<String> = line.split(",").map(|s| s.trim().to_string()).collect();
            records.push(row);
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_csv_reader_read_success() {
        let file_path = "test.csv";
        {
            let mut file = File::create(file_path).expect("failed to create test file");
            writeln!(file, "id, team_id, name, note").expect("failed to create test file");
            writeln!(file, "1, 1, name1, note1").expect("failed to create test file");
            writeln!(file, "2, 1, name2, note2").expect("failed to create test file");
        }

        let reader = CSVReader::new(file_path.to_string());
        let records = reader.read().expect("faild to read csv file");

        assert_eq!(records.len(), 3);
        assert_eq!(records[0], vec!["id", "team_id", "name", "note"]);
        assert_eq!(records[1], vec!["1", "1", "name1", "note1"]);
        assert_eq!(records[2], vec!["2", "1", "name2", "note2"]);

        std::fs::remove_file(file_path).expect("failed to delete test file")
    }
}
