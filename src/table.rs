use crate::row::Row;

pub struct Table {
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: Row) {
        self.rows.push(row);
    }

    #[allow(dead_code)]
    pub fn select(&self, id: u32) -> Option<&Row> {
        for row in &self.rows {
            if row.id == id {
                return Some(row);
            }
        }
        None
    }

    pub fn get_all_rows(&self) -> &Vec<Row> {
        &self.rows
    }
}