use super::query::{Query, QueryFrom};

impl QueryFrom for Query {
    fn get_file_path(&self) -> Result<String, String> {
        let mut file_path: Option<String> = None;
        for (i, v) in self.clauses.iter().enumerate() {
            if v == "FROM" && i + 1 < self.clauses.len() {
                file_path = Some(self.clauses[i + 1].clone());
                break;
            }
        }
        match file_path {
            Some(path) => Ok(path),
            None => Err("syntax error: FROM must specify a file path".to_string()),
        }
    }
}
