use std::collections::HashSet;

trait QueryBase {
    fn validate(&self) -> Result<(), String>;
}

pub trait QueryFrom {
    fn get_file_path(&self) -> Result<String, String>;
}

pub trait QuerySelect {
    fn get_select(&self) -> Vec<String>;
}


#[derive(Debug)]
pub struct Query {
    pub clauses: Vec<String>,
}

impl Query {
    pub fn new(input: &str) -> Self {
        let clauses = parse_clauses(input);
        Query { clauses }
    }

    pub fn validate(&self) -> Result<(), String> {
        QueryBase::validate(self)
    }

    pub fn get_file_path(&self) -> Result<String, String> {
        QueryFrom::get_file_path(self)
    }

    pub fn get_select(&self) -> Vec<String> {
        QuerySelect::get_select(self)
    }
}

impl QueryBase for Query {
    fn validate(&self) -> Result<(), String> {
        if self.clauses.is_empty() || self.clauses[0] != "SELECT" {
            return Err("syntax error: query must start with SELECT".to_string());
        }

        Ok(())
    }
}

fn parse_clauses(input: &str) -> Vec<String> {
    let mut clauses = Vec::new();
    let mut current_token = String::new();
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '"' => in_quote = !in_quote,
            ',' if in_quote => {
                append_token(&mut clauses, &mut current_token);
                clauses.push(", ".to_string());
            }
            ' ' if !in_quote => {
                append_token(&mut clauses, &mut current_token);
            }
            _ => {
                if c != ',' {
                    current_token.push(c)
                }
            }
        }
    }

    append_token(&mut clauses, &mut current_token);
    clauses
}

fn append_token(clauses: &mut Vec<String>, token: &mut String) {
    if !token.is_empty() {
        let normarized = if is_reserved_word(token) {
            token.to_uppercase()
        } else {
            token.clone()
        };
        clauses.push(normarized);
        token.clear();
    }
}

fn is_reserved_word(word: &str) -> bool {
    let reserved: HashSet<&str> = ["select", "from"].iter().cloned().collect();
    reserved.contains(&word.to_lowercase().as_str())
}

#[cfg(test)]
mod test_query {
    use super::*;

    #[test]
    fn test_query_validate_succeed() {
        let query = Query::new("select column from ./sample.csv");
        assert!(query.validate().is_ok())
    }

    #[test]
    fn test_query_validate_failed() {
        let query = Query::new("");
        assert_eq!(
            query.validate(),
            Err("syntax error: query must start with SELECT".to_string())
        );
    }

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

    #[test]
    fn test_get_select() {
        let query = Query::new("select column1, column2 from ./sample.csv");
        let result = query.get_select();
        let expected = vec!["column1".to_string(), "column2".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_select_with_comma() {
        let query = Query::new("select column1 , column2 from ./sample.csv");
        let result = query.get_select();
        let expected = vec!["column1".to_string(), "column2".to_string()];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_query_helper {
    use super::*;

    #[test]
    fn test_parse_clauses_with_quotes() {
        let input1 = r#"select column1 , column2 from ./sample.csv"#;
        let expected = vec![
            "SELECT".to_string(),
            "column1".to_string(),
            "column2".to_string(),
            "FROM".to_string(),
            "./sample.csv".to_string(),
        ];
        assert_eq!(parse_clauses(input1), expected);

        let input2 = r#"select column1, column2 from ./sample.csv"#;
        let expected = vec![
            "SELECT".to_string(),
            "column1".to_string(),
            "column2".to_string(),
            "FROM".to_string(),
            "./sample.csv".to_string(),
        ];
        assert_eq!(parse_clauses(input2), expected);
    }

    #[test]
    fn test_append_token_with_reserved_word() {
        let mut clauses: Vec<String> = Vec::new();
        let mut token = "select".to_string();

        append_token(&mut clauses, &mut token);
        assert_eq!(clauses, vec!["SELECT"]);
        assert!(token.is_empty());
    }

    #[test]
    fn test_append_token_with_non_reserved_word() {
        let mut clauses: Vec<String> = Vec::new();
        let mut token = "column".to_string();

        append_token(&mut clauses, &mut token);
        assert_eq!(clauses, vec!["column"]);
        assert!(token.is_empty());
    }

    #[test]
    fn test_append_token_with_empty_token() {
        let mut clauses: Vec<String> = Vec::new();
        let mut token = "".to_string();

        append_token(&mut clauses, &mut token);
        assert!(clauses.is_empty());
        assert!(token.is_empty());
    }

    #[test]
    fn test_is_reserved_word() {
        assert_eq!(is_reserved_word("SELECT"), true);
        assert_eq!(is_reserved_word("FROM"), true);
        assert_eq!(is_reserved_word("column"), false);
    }
}
