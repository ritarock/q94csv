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

#[cfg(test)]
mod test_query_from {
    use super::*;

    #[test]
    fn test_get_file_path() {
        let query = Query::new("select column from ./sample.csv");
        let result = query.get_file_path();
        assert_eq!(result, Ok("./sample.csv".to_string()));
    }

    #[test]
    fn test_get_file_path_with_missing_from() {
        let query = Query::new("select column ./sample.csv");
        let result = query.get_file_path();
        assert_eq!(
            result,
            Err("syntax error: FROM must specify a file path".to_string())
        );
    }

    #[test]
    fn test_get_file_path_with_missing_file_path() {
        let query = Query::new("select column from");
        let result = query.get_file_path();
        assert_eq!(
            result,
            Err("syntax error: FROM must specify a file path".to_string())
        );
    }
}
