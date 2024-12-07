use std::collections::HashMap;
use std::fmt::Debug;
use crate::columns::column_trait::ColumnTrait;
use crate::columns::string_column::StringColumn;
use crate::columns::float_column::FloatColumn;

pub struct Table {
    pub columns: Vec<Box<dyn ColumnTrait>>,
}

impl Table {
    pub fn new() -> Self {
        Self { columns: Vec::new() }
    }

    pub fn add_column(&mut self, col: Box<dyn ColumnTrait>) {
        // Ensure unique name
        let name = col.name().to_string();
        if self.columns.iter().any(|c| c.name() == name) {
            panic!("Column with name {} already exists", name);
        }
        self.columns.push(col);
    }

    pub fn add_string_column(&mut self, name: &str) {
        self.add_column(Box::new(StringColumn::new(name)));
    }

    pub fn add_float_column(&mut self, name: &str) {
        self.add_column(Box::new(FloatColumn::new(name)));
    }

    pub fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|c| c.name()).collect()
    }

    pub fn get_column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name() == name)
    }

    pub fn len(&self) -> usize {
        if self.columns.is_empty() {
            0
        } else {
            self.columns[0].len()
        }
    }

    pub fn push_row_str(&mut self, values: &HashMap<&str, String>) {
        // values: column_name -> value as str
        // ensure all columns are present
        for c in &self.columns {
            let col_name = c.name();
            if !values.contains_key(col_name) {
                panic!("Missing value for column {}", col_name);
            }
        }
        for c in self.columns.iter_mut() {
            c.push_str(values.get(c.name()).unwrap());
        }
    }

    pub fn create_index(&self, column: &str) -> HashMap<String, Vec<usize>> {
        let idx = self.get_column_index(column).expect("Index column not found");
        let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        for i in 0..self.len() {
            let key = self.columns[idx].get_as_string(i);
            map.entry(key).or_default().push(i);
        }
        map
    }

}

impl Clone for Table {
    fn clone(&self) -> Self {
        let cloned_columns = self.columns.iter()
            .map(|c| c.clone_box())
            .collect();
        Self {
            columns: cloned_columns,
        }
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print table as a markdown table
        let mut table = String::new();
        let header = self.column_names();
        let rows = (0..self.len())
            .map(|i| {
                header.iter()
                    .map(|c| self.columns[self.get_column_index(c).unwrap()].get_as_string(i))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        // print header
        table.push_str("|");
        for h in header.as_slice() {
            table.push_str(&format!(" {} |", h));
        }
        table.push_str("\n");

        // print separator
        table.push_str("|");
        for _ in header.as_slice() {
            table.push_str(" --- |");
        }
        table.push_str("\n");

        // print rows
        for row in rows {
            table.push_str("|");
            for cell in row {
                table.push_str(&format!(" {} |", cell));
            }
            table.push_str("\n");
        }

        write!(f, "{}", table)
    }
}