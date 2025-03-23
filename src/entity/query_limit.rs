use super::query::{Query, QueryLimit};

impl QueryLimit for Query {
    fn get_limit(&self) -> u32 {
        let mut limit_index = -1;

        for (i, v) in self.clauses.iter().enumerate() {
            if v == "LIMIT" {
                limit_index = i as isize;
                break;
            }
        }

        if limit_index == -1 || limit_index as usize + 1 >= self.clauses.len() {
            return 0;
        }

        match self.clauses[(limit_index as usize) + 1].parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod test_query_limit {
    use super::*;

    #[test]
    fn test_get_limit_with_valid_limit() {
        let query = Query::new("select column from ./sample.csv limit 10");
        assert_eq!(query.get_limit(), 10);
    }

    #[test]
    fn test_get_limit_without_limit() {
        let query = Query::new("select column from ./sample.csv");
        assert_eq!(query.get_limit(), 0);
    }

    #[test]
    fn test_get_limit_with_invalid_limit() {
        let query = Query::new("select column from ./sample.csv limit abc");
        assert_eq!(query.get_limit(), 0);
    }
}
