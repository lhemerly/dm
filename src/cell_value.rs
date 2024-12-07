pub enum CellValue {
    Int(i64),
    Float(f64),
    Text(String),
    DateTime(i64), // Store as timestamp
}

impl CellValue {
    pub fn as_string(&self) -> String {
        match self {
            CellValue::Int(i) => i.to_string(),
            CellValue::Float(f) => f.to_string(),
            CellValue::Text(s) => s.clone(),
            CellValue::DateTime(dt) => dt.to_string(),
        }
    }
}
