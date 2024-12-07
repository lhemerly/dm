use std::collections::HashMap;
use crate::table::Table;

pub fn group_by(table: &Table, on: &str) -> HashMap<String, Vec<usize>> {
    let idx = table.get_column_index(on).expect("Group column not found");
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for i in 0..table.len() {
        let key = table.columns[idx].get_as_string(i);
        map.entry(key).or_default().push(i);
    }
    map
}
