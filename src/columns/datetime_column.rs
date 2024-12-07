use crate::columns::column_trait::ColumnTrait;

#[derive(Clone)]
pub struct DateTimeColumn {
    name: String,
    data: Vec<i64>, // UNIX timestamps
}

impl DateTimeColumn {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, val: i64) {
        self.data.push(val);
    }
    pub fn get(&self, idx: usize) -> i64 {
        self.data[idx]
    }
}

impl ColumnTrait for DateTimeColumn {
    fn name(&self) -> &str {
        &self.name
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn get_as_string(&self, idx: usize) -> String {
        self.data[idx].to_string()
    }
    fn push_str(&mut self, val: &str) {
        let parsed = val.parse::<i64>().expect("Invalid datetime (timestamp) value");
        self.data.push(parsed);
    }
    fn clone_box(&self) -> Box<dyn ColumnTrait> {
        Box::new(self.clone())
    }
}
