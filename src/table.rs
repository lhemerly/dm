use std::collections::HashMap;
use std::fmt::Debug;
use crate::columns::column_trait::ColumnTrait;
use crate::columns::string_column::StringColumn;
use crate::columns::float_column::FloatColumn;
use crate::columns::column_trait::ColumnType;

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

    pub fn get_rows(&self) -> Vec<HashMap<&str, String>> {
        let mut rows = Vec::new();
        for i in 0..self.len() {
            let mut row = HashMap::new();
            for c in &self.columns {
                row.insert(c.name(), c.get_as_string(i));
            }
            rows.push(row);
        }
        rows
    }

    pub fn push_row(&mut self, values: &HashMap<&str, String>) {
        // values: column_name -> value as str
        // ensure all columns are present
        for c in self.columns.iter_mut() {
            let col_name = c.name();
            if !values.contains_key(col_name) {
                // Match c type and add default value
                match c.get_type() {
                    ColumnType::String => c.push_str(""),
                    ColumnType::Int => c.push_str("0"),
                    ColumnType::Float => c.push_str("0.0"),
                }
            } else {
                c.push_str(values.get(col_name).unwrap());
            }
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

    pub fn create_multi_index(&self, columns: &[&str]) -> HashMap<Vec<String>, Vec<usize>> {
        let idxs: Vec<usize> = columns.iter().map(|&col| self.get_column_index(col).expect("Index column not found")).collect();
        let mut map: HashMap<Vec<String>, Vec<usize>> = HashMap::new();
        for i in 0..self.len() {
            let key: Vec<String> = idxs.iter().map(|&idx| self.columns[idx].get_as_string(i)).collect();
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