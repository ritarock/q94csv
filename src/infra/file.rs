use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct CSVReader;

impl CSVReader {
    pub fn new() -> Self {
        CSVReader
    }

    pub fn read(&self, file_path: &str) -> Result<Vec<Vec<String>>, String> {
        let file = File::open(file_path).map_err(|err| err.to_string())?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    let row: Vec<String> = l.split(",").map(|s| s.trim().to_string()).collect();
                    records.push(row);
                }
                Err(err) => return Err(err.to_string()),
            }
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_csv_reader_read() {
        let file_path = "test.csv";
        {
            let mut file = File::create(file_path).expect("failed to create test file");
            writeln!(file, "id, team_id, name, note").expect("failed to create test file");
            writeln!(file, "1, 1, name1, note1").expect("failed to create test file");
            writeln!(file, "2, 1, name2, note2").expect("failed to create test file");
        }

        let reader = CSVReader::new();
        let records = reader.read(file_path).expect("faild to read csv file");

        assert_eq!(records.len(), 3);
        assert_eq!(records[0], vec!["id", "team_id", "name", "note"]);
        assert_eq!(records[1], vec!["1", "1", "name1", "note1"]);
        assert_eq!(records[2], vec!["2", "1", "name2", "note2"]);

        std::fs::remove_file(file_path).expect("failed to delete test file")
    }
}
