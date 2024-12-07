use std::collections::HashMap;
use crate::table::Table;

/// Perform an inner join on two tables based on a key column.
///
/// # Arguments
///
/// * `left` - The left table
/// * `right` - The right table
/// * `on` - The column name on which to join
pub fn inner_join(left: &Table, right: &Table, on: &str) -> Table {
    let left_idx = left.get_column_index(on).expect("Join column not found in left table");
    let right_idx = right.create_index(on);

    let key = left.columns[left_idx].get_as_string(0);

    let mut result = Table::new();
    if let Some(matches) = right_idx.get(&key) {
        for i in matches {
            let mut row: HashMap<&str, String> = HashMap::new();
            for (_idx, c) in left.columns.iter().enumerate() {
                row.insert(c.name(), c.get_as_string(0));
            }
            result.push_row_str(&row);
        }
    }

    result
}