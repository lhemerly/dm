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
        table.push_row(&row);
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

    let mut table2 = Table::new();

    table2.add_column(Box::new(StringColumn::new("product")));
    table2.add_column(Box::new(StringColumn::new("description")));

    let rows2 = vec![
        HashMap::from([
            ("product", "A".to_string()),
            ("description", "Product A".to_string()),
        ]),
        HashMap::from([
            ("product", "B".to_string()),
            ("description", "Product B".to_string()),
        ])
    ];

    for row in rows2 {
        table2.push_row(&row);
    }

    // columns to join on
    let on = vec!["product"];

    // type to join
    let join_type = dm::methods::join::JoinType::Left;

    // perform inner join
    let joined = dm::methods::join::join(&table, &table2, &on, join_type);

    println!("\nJoined table:");
    println!("{:?}", joined);

}
