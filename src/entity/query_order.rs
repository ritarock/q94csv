use super::query::{Order, Query, QueryOrder};

impl QueryOrder for Query {
    fn get_order(&self) -> Order {
        let mut order = Order {
            column: String::new(),
            condition: String::from("ASC"),
        };
        let mut order_index = -1;

        for (i, v) in self.clauses.iter().enumerate() {
            if v == "ORDER" {
                order_index = i as isize;
                break;
            }
        }

        if order_index > 0 {
            order.column = self.clauses[(order_index as usize) + 2].clone();

            if self.clauses.len() == (order_index as usize) + 2 {
                return order;
            }

            match self
                .clauses
                .get((order_index as usize) + 3)
                .map(|s| s.as_str())
            {
                Some("ASC") => order.condition = "ASC".to_string(),
                Some("DESC") => order.condition = "DESC".to_string(),
                _ => order.condition = "ASC".to_string(),
            }
        }

        order
    }
}

#[cfg(test)]
mod test_query_order {
    use super::*;

    #[test]
    fn test_get_order_with_order_clause_asc() {
        let query = Query::new("select column from ./sample.csv order by id asc");

        let expected: Order = Order {
            column: "id".to_string(),
            condition: "ASC".to_string(),
        };

        let result = query.get_order();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_order_with_order_clause_desc() {
        let query = Query::new("select column from ./sample.csv order by id desc");

        let expected: Order = Order {
            column: "id".to_string(),
            condition: "DESC".to_string(),
        };

        let result = query.get_order();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_order_with_order_clause_default() {
        let query = Query::new("select column from ./sample.csv order by id");

        let expected: Order = Order {
            column: "id".to_string(),
            condition: "ASC".to_string(),
        };

        let result = query.get_order();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_order_with_no_order_clause() {
        let query = Query::new("select column from ./sample.csv");

        let expected: Order = Order {
            column: String::new(),
            condition: "ASC".to_string(),
        };

        let result = query.get_order();

        assert_eq!(result, expected);
    }
}
