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
