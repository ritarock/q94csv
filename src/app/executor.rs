use crate::{
    entity::{
        query::Query,
        record::{Records, limit_rows},
    },
    infra::file::CSVReader,
};

pub struct Executor {
    pub csv_reader: CSVReader,
}

#[derive(Debug)]
pub struct Results {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Executor {
    pub fn new(csv_reader: CSVReader) -> Self {
        Executor { csv_reader }
    }

    pub fn execute(&self, query: Query) -> Result<Results, String> {
        let file_path = query.get_file_path().map_err(|err| err)?;
        let read_csv = self.csv_reader.read(&file_path).map_err(|err| err)?;
        let records: Records = read_csv;

        if records.len() == 0 {
            return Ok(Results {
                header: Vec::new(),
                rows: Vec::new(),
            });
        }

        let select = query.get_select();
        let headers = &records[0];
        let mut column_indices: Vec<usize> = Vec::new();

        if select.len() == 1 && select[0] == "*" {
            column_indices = (0..headers.len()).collect();
        } else {
            for col in select {
                for (i, header) in headers.iter().enumerate() {
                    if col.eq_ignore_ascii_case(&header) {
                        column_indices.push(i);
                        break;
                    }
                }
            }
        }

        let limit = query.get_limit();
        let records = if limit > 0 {
            limit_rows(&records, limit)
        } else {
            records.clone()
        };

        let mut result = Results {
            header: column_indices.iter().map(|&i| headers[i].clone()).collect(),
            rows: Vec::new(),
        };

        for row in records.iter().skip(1) {
            let mut selected_row = Vec::new();
            for &idx in &column_indices {
                selected_row.push(row[idx].clone());
            }
            result.rows.push(selected_row);
        }

        Ok(result)
    }
}
