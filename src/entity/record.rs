pub type Records = Vec<Vec<String>>;

pub struct Record;

impl Record {
    pub fn limit_rows(records: &Records, limit: u32) -> Records {
        let mut result = records.clone();
        result.truncate(limit as usize + 1);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_limit_rows() {
        let records: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["3".to_string(), "name3".to_string()],
        ];

        let result = Record::limit_rows(&records, 1);

        assert_eq!(result.len(), 2);
        assert_eq!(result[1], vec!["1".to_string(), "name1".to_string()]);
    }
}
