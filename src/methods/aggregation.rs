use crate::table::Table;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum AggregationType {
    Sum,
    Count,
    // Add more (Avg, Min, Max) as needed
}

pub fn aggregate(table: &Table, groups: &HashMap<String, Vec<usize>>, column: &str, agg_type: AggregationType) -> HashMap<String, f64> {
    let col_idx = table.get_column_index(column).expect("Aggregation column not found");
    let c = &table.columns[col_idx];

    // For simplicity, parse values as floats
    let mut result = HashMap::new();

    for (key, rows) in groups {
        match agg_type {
            AggregationType::Count => {
                result.insert(key.clone(), rows.len() as f64);
            }
            AggregationType::Sum => {
                let mut sum = 0.0;
                for &r in rows {
                    let val_str = c.get_as_string(r);
                    let val: f64 = val_str.parse().expect("Non-numeric value in sum");
                    sum += val;
                }
                result.insert(key.clone(), sum);
            }
        }
    }

    result
}
