use super::query::{Query, QueryWhereCondition, WhereCondition};

impl QueryWhereCondition for Query {
    fn get_where(&self) -> WhereCondition {
        let mut conditions: WhereCondition = Vec::new();
        let mut where_index = -1;

        for (i, v) in self.clauses.iter().enumerate() {
            if v == "WHERE" {
                where_index = i as isize;
                break;
            }
        }

        if where_index > 0 {
            let where_clauses = &self.clauses[where_index as usize..];
            let mut where_vec: Vec<String> = Vec::new();

            for v in where_clauses {
                if v == "AND" {
                    continue;
                }
                where_vec.push(v.clone());
            }

            let mut i = 0;
            while i < where_vec.len() {
                if where_vec[i] == "ORDER" {
                    break;
                }
                if i + 2 >= where_vec.len() {
                    break;
                }

                let mut current: Vec<String> = Vec::new();
                current.push(where_vec[i + 1].clone());
                current.push(where_vec[i + 2].clone());
                current.push(where_vec[i + 3].clone());

                conditions.push(current);
                i += 4;
            }
        }

        conditions
    }
}

#[cfg(test)]
mod test_query_where {
    use super::*;

    #[test]
    fn test_get_where() {
        let query = Query::new("select column from ./sample.csv where id = 3 and name = name3");
        let result = query.get_where();
        let expected = vec![
            vec!["id".to_string(), "=".to_string(), "3".to_string()],
            vec!["name".to_string(), "=".to_string(), "name3".to_string()],
        ];
        assert_eq!(result, expected);
    }
}
