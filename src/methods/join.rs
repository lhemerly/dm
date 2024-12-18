use std::collections::{HashMap, HashSet};
use crate::table::Table;
use crate::columns::string_column::StringColumn;

pub enum JoinType {
    Inner,
    Left,
    Right,
}

/// Perform a join on two tables based on key columns.
///
///
///
/// # Arguments
///
/// * `left` - The left table
/// * `right` - The right table
/// * `on` - The column names on which to join
/// * `join_type` - The type of join to perform
pub fn join(left: &Table, right: &Table, on: &[&str], join_type: JoinType) -> Table {
    // Get indexes
    let left_on_idxs: Vec<usize> = on.iter().map(|&col| left.get_column_index(col).expect("Join column not found in left table")).collect();
    let right_index = right.create_multi_index(on);

    // Collect all columns from both tables
    let mut all_columns: HashSet<String> = HashSet::new();
    for col in left.columns.iter() {
        all_columns.insert(col.name().to_string());
    }
    for col in right.columns.iter() {
        all_columns.insert(col.name().to_string());
    }

    // Create a vector of column names preserving order (if needed)
    let all_columns: Vec<String> = all_columns.into_iter().collect();

    // Create the result table with all columns, preserving types
    let mut result = Table::new();
    for col_name in &all_columns {
        result.add_column(Box::new(StringColumn::new(col_name)));
    }

    // Iterate over the left table
    for i in 0..left.len() {
        let key: Vec<String> = left_on_idxs.iter().map(|&idx| left.columns[idx].get_as_string(i)).collect();
        // Check if the key is present in the right table
        if let Some(right_rows) = right_index.get(&key) {
            // Create a row for each match
            for right_row in right_rows {
                let mut row = HashMap::new();
                for col_name in &all_columns {
                    if let Some(col_idx) = left.get_column_index(col_name) {
                        row.insert(col_name.as_str(), left.columns[col_idx].get_as_string(i));
                    } else if let Some(col_idx) = right.get_column_index(col_name) {
                        row.insert(col_name.as_str(), right.columns[col_idx].get_as_string(*right_row));
                    }
                }
                result.push_row(&row);
            }
        } else if matches!(join_type, JoinType::Left) {
            // For left join, add row with left table data and nulls for right table data
            let mut row = HashMap::new();
            for col_name in &all_columns {
                if let Some(col_idx) = left.get_column_index(col_name) {
                    row.insert(col_name.as_str(), left.columns[col_idx].get_as_string(i));
                } else {
                    row.insert(col_name.as_str(), String::new()); // Assuming empty string for nulls
                }
            }
            result.push_row(&row);
        }
    }

    if matches!(join_type, JoinType::Right) {
        // For right join, add rows from right table that do not have matches in left table
        for (key, right_rows) in right_index.iter() {
            let left_rows: Vec<_> = (0..left.len()).filter(|&i| {
                let left_key: Vec<String> = left_on_idxs.iter().map(|&idx| left.columns[idx].get_as_string(i)).collect();
                &left_key == key
            }).collect();
            if left_rows.is_empty() {
                for right_row in right_rows {
                    let mut row = HashMap::new();
                    for col_name in &all_columns {
                        if let Some(_col_idx) = left.get_column_index(col_name) {
                            row.insert(col_name.as_str(), String::new()); // Assuming empty string for nulls
                        } else if let Some(col_idx) = right.get_column_index(col_name) {
                            row.insert(col_name.as_str(), right.columns[col_idx].get_as_string(*right_row));
                        }
                    }
                    result.push_row(&row);
                }
            }
        }
    }

    result
}