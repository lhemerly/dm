use std::collections::HashMap;
use crate::table::Table;
use crate::methods::aggregation::AggregationType;

/// Create a pivot table.
/// 
/// # Arguments
/// 
/// * `table` - the original table
/// * `row_groups` - columns to group for rows
/// * `col_groups` - columns to group for columns
/// * `value_col` - column name to aggregate
/// * `agg_type` - aggregation function (Sum, Count, etc.)
///
/// # Returns
///
/// A new `Table` that represents the pivoted data.
///
/// # Example
///
/// Suppose you have a table with columns: ["region", "product", "sales"].
///
/// - rows: ["region"]
/// - cols: ["product"]
/// - value_col: "sales"
/// - agg_type: Sum
///
/// If you have:
///
/// | region | product | sales |
/// |--------|----------|-------|
/// | North  | A        | 10    |
/// | North  | B        | 20    |
/// | South  | A        | 5     |
///
/// The pivot might look like:
///
/// | region | A    | B    |
/// |--------|------|------|
/// | North  | 10   | 20   |
/// | South  | 5    | 0    |
///
pub fn pivot(
    table: &Table,
    row_groups: &[&str],
    col_groups: &[&str],
    value_col: &str,
    agg_type: AggregationType,
) -> Table {
    let value_idx = table.get_column_index(value_col).expect("Value column not found");

    // Get indexes for row and column grouping
    let row_idxs: Vec<usize> = row_groups.iter().map(|&rg| table.get_column_index(rg).expect("Row group column not found")).collect();
    let col_idxs: Vec<usize> = col_groups.iter().map(|&cg| table.get_column_index(cg).expect("Col group column not found")).collect();

    // Helper to build keys
    fn make_key(table: &Table, idxs: &[usize], row: usize) -> String {
        let mut parts = Vec::new();
        for &i in idxs {
            parts.push(table.columns[i].get_as_string(row));
        }
        parts.join("|") // Join with a delimiter to form a composite key
    }

    // Data structure: row_key -> col_key -> Vec<row_index>
    let mut groups: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();

    for i in 0..table.len() {
        let rkey = make_key(table, &row_idxs, i);
        let ckey = make_key(table, &col_idxs, i);

        groups.entry(rkey).or_default().entry(ckey).or_default().push(i);
    }

    // Extract all unique col keys to build column headers in output
    let mut col_key_set = std::collections::HashSet::new();
    for cmap in groups.values() {
        for ckey in cmap.keys() {
            col_key_set.insert(ckey.clone());
        }
    }

    let mut col_keys: Vec<String> = col_key_set.into_iter().collect();
    col_keys.sort();

    // Prepare output table:
    // Row grouping columns first
    // Then one column per col_key for the aggregated values
    let mut out = Table::new();

    // Add columns for row groups
    for &rg in row_groups {
        // For now, let's just use StringColumn for simplicity
        out.add_string_column(rg); 
    }

    // Add columns for each unique col_key
    for ck in &col_keys {
        // name could be the col grouping keys combined plus the agg function name
        let col_name = format!("{}_{:?}", ck, agg_type);
        out.add_float_column(&col_name);
    }

    // Perform aggregation and fill data into `out`.
    // We'll also need to figure out the unique row keys sorted, so we have stable ordering.
    let mut row_key_list: Vec<String> = groups.keys().cloned().collect();
    row_key_list.sort();

    for rk in &row_key_list {
        let row_map: &HashMap<String, Vec<usize>> = &groups[rk];

        // Parse the row key back into the individual row-group values
        let rg_values: Vec<&str> = rk.split('|').collect();

        // We'll build a HashMap<&str, &str> for the row and its aggregated values
        // Actually we know the schema, so let's insert row keys first as strings.
        // Then we will insert the aggregated values (floats) after.

        // Make a map to fill in and then call push_row_str at the end.
        // But we have float columns for the aggregated values, so we need them as strings.
        let mut row_values: HashMap<String, String> = HashMap::new();

        // Insert row group values
        for (rg_col, rg_val) in row_groups.iter().zip(rg_values.iter()) {
            row_values.insert(rg_col.to_string(), rg_val.to_string());
        }

        // For each col key, aggregate
        for ck in &col_keys {
            let rows_for_ck: Option<&Vec<usize>> = row_map.get(ck);
            let agg_result: f64 = if let Some(ridxs) = rows_for_ck {
                aggregate_rows(table, ridxs, value_idx, agg_type)
            } else {
                // no rows for this combination
                0.0
            };

            // Identify the output column name
            let col_name: String = format!("{}_{:?}", ck, agg_type);
            row_values.insert(col_name, agg_result.to_string());
        }
        
        row_values.insert("row_key".to_string(), rk.clone());

        // Create a temporary HashMap<&str, String>
        let temp_row_values: HashMap<&str, String> = row_values.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();

        out.push_row(&temp_row_values);
    }

    out
}

/// Helper function to aggregate a set of rows from the value column using a specific aggregation.
fn aggregate_rows(table: &Table, rows: &[usize], value_idx: usize, agg_type: AggregationType) -> f64 {
    match agg_type {
        AggregationType::Count => rows.len() as f64,
        AggregationType::Sum => {
            let mut sum = 0.0;
            for &r in rows {
                let val_str = table.columns[value_idx].get_as_string(r);
                let val: f64 = val_str.parse().expect("Non-numeric value in sum");
                sum += val;
            }
            sum
        }
    }
}
