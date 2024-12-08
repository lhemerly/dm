use crate::columns::column_trait::ColumnTrait;
use crate::columns::column_trait::ColumnType;

#[derive(Clone)]
pub struct StringColumn {
    name: String,
    data: Vec<String>,
}

impl StringColumn {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, val: &str) {
        self.data.push(val.to_string());
    }
    pub fn get(&self, idx: usize) -> &str {
        &self.data[idx]
    }
}

impl ColumnTrait for StringColumn {
    fn name(&self) -> &str {
        &self.name
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get_as_string(&self, idx: usize) -> String {
        self.data[idx].clone()
    }
    fn push_str(&mut self, val: &str) {
        self.data.push(val.to_string());
    }
    fn clone_box(&self) -> Box<dyn ColumnTrait> {
        Box::new(self.clone())
    }
    fn get_type(&self) -> ColumnType {
        ColumnType::String
    }
}
