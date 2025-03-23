use super::query::{Query, QuerySelect};

impl QuerySelect for Query {
    fn get_select(&self) -> Vec<String> {
        let mut columns = Vec::new();
        for clause in self.clauses.iter().skip(1) {
            if clause == "FROM" {
                break;
            }
            if clause != "," {
                let col = clause.clone();
                columns.push(col);
            }
        }
        columns
    }
}

#[cfg(test)]
mod test_query_select {
    use super::*;

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
