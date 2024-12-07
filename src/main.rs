use dm::table::Table;
use dm::columns::float_column::FloatColumn;
use dm::columns::string_column::StringColumn;
use dm::methods::aggregation::AggregationType;
use dm::methods::pivot;
use std::collections::HashMap;

fn main() {
    // Create a new table
    let mut table = Table::new();
    
    // Add columns
    table.add_column(Box::new(StringColumn::new("region")));
    table.add_column(Box::new(StringColumn::new("product"))); 
    table.add_column(Box::new(FloatColumn::new("sales")));

    // Add sample data
    let rows = vec![
        HashMap::from([
            ("region", "North".to_string()),
            ("product", "A".to_string()),
            ("sales", "10".to_string()),
        ]),
        HashMap::from([
            ("region", "North".to_string()),
            ("product", "A".to_string()),
            ("sales", "20".to_string()),
        ]),
        HashMap::from([
            ("region", "South".to_string()),
            ("product", "B".to_string()),
            ("sales", "5".to_string()),
        ]),
        HashMap::from([
            ("region", "South".to_string()),
            ("product", "B".to_string()),
            ("sales", "10".to_string()),
        ]),
    ];

    for row in rows {
        table.push_row_str(&row);
    }

    // Create pivot table
    let pivoted = pivot::pivot(
        &table,
        &["region"],           // row groups 
        &["product"],          // column groups
        "sales",              // value column
        AggregationType::Sum  // aggregation type
    );

    // Print both tables to terminal
    println!("Original table:");
    println!("{:?}", table);
    println!("\nPivoted table:");
    println!("{:?}", pivoted);
}
