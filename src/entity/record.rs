use super::query::{Order, WhereCondition};

pub type Records = Vec<Vec<String>>;

pub struct Record;

impl Record {
    pub fn limit_rows(records: &Records, limit: u32) -> Records {
        let mut result = records.clone();
        result.truncate(limit as usize + 1);
        result
    }

    pub fn sort_rows(records: &Records, order: &Order) -> Records {
        let mut column_index = -1;

        for (i, header) in records[0].iter().enumerate() {
            if header == &order.column {
                column_index = i as isize;
                break;
            }
        }

        if column_index == -1 {
            return records.clone();
        }

        let is_asc = order.condition == "ASC";
        let header = records[0].clone();
        let mut rows = records[1..].to_vec();
        rows.sort_by(|a, b| {
            let cmp = a[column_index as usize].cmp(&b[column_index as usize]);
            if is_asc { cmp } else { cmp.reverse() }
        });

        rows.insert(0, header);

        rows as Records
    }

    pub fn fileter_rows(records: &Records, conditions: &WhereCondition) -> Records {
        let mut filtered = records.clone();
        for condition in conditions {
            filtered = filter_row_condition(filtered, condition)
        }

        filtered
    }
}

fn filter_row_condition(r: Records, condition: &Vec<String>) -> Records {
    if r.is_empty() {
        return r;
    }

    let (column, operator, value) = (&condition[0], &condition[1], &condition[2]);
    let mut column_index = -1;

    let header = &r[0];
    for (i, v) in header.iter().enumerate() {
        if v == column {
            column_index = i as isize;
            break;
        }
    }

    if column_index == -1 {
        return r;
    }

    let mut fileterd = vec![header.clone()];

    for row in &r[1..] {
        if row.len() <= column_index as usize {
            continue;
        }

        let cell_value = &row[column_index as usize];
        let mut match_condition = false;

        match operator.as_str() {
            "=" => match_condition = cell_value == value,
            "!=" => match_condition = cell_value != value,
            "<" | ">" | "<=" | ">=" => {
                let cell_num = cell_value.parse::<f64>();
                let value_num = cell_value.parse::<f64>();

                match (cell_num, value_num) {
                    (Ok(cell), Ok(val)) => match operator.as_str() {
                        "<" => match_condition = cell < val,
                        ">" => match_condition = cell > val,
                        "<=" => match_condition = cell <= val,
                        ">=" => match_condition = cell >= val,
                        _ => (),
                    },
                    _ => (),
                }
            }
            _ => (),
        }

        if match_condition {
            fileterd.push(row.clone());
        }
    }

    fileterd
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

    #[test]
    fn test_sort_rows_asc() {
        let records: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["3".to_string(), "name3".to_string()],
        ];

        let order: Order = Order {
            column: "id".to_string(),
            condition: "ASC".to_string(),
        };

        let result = Record::sort_rows(&records, &order);
        let expected: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["3".to_string(), "name3".to_string()],
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn test_sort_rows_desc() {
        let records: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["3".to_string(), "name3".to_string()],
        ];

        let order: Order = Order {
            column: "id".to_string(),
            condition: "DESC".to_string(),
        };

        let result = Record::sort_rows(&records, &order);
        let expected: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["3".to_string(), "name3".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["1".to_string(), "name1".to_string()],
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn test_fileter_rows() {
        let records: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
            vec!["2".to_string(), "name2".to_string()],
            vec!["3".to_string(), "name3".to_string()],
        ];

        let conditions: WhereCondition = vec![
            vec!["id".to_string(), "=".to_string(), "2".to_string()],
        ];

        let result = Record::fileter_rows(&records, &conditions);
        let expected: Records = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["2".to_string(), "name2".to_string()],
        ];

        assert_eq!(result, expected)
    }
}
