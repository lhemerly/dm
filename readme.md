# DM - Data Manager

DM (Data Manager) is a fast and efficient tool designed for big data treatment. It provides a set of powerful methods to manipulate and analyze large datasets with ease. Whether you need to perform complex joins, aggregations, or pivot operations, DM has you covered.

### Features

- *Inner Join*: Perform inner joins on tables based on key columns.
- *Aggregation*: Aggregate data using various functions like sum and count.
- *Pivot*: Create pivot tables to summarize and analyze data.
- *Flexible Data Types*: Support for integer, float, string, and datetime columns.

### Installation
To use DM, you need to have Rust installed. You can install Rust from rust-lang.org.

Clone the repository and build the project:
```
git clone https://github.com/yourusername/dm.git
cd dm
cargo build --release
```

### Usage

#### Creating a Table

You can create a new table and add columns to it:
```
use dm::table::Table;
use dm::columns::string_column::StringColumn;
use dm::columns::float_column::FloatColumn;

let mut table = Table::new();
table.add_column(Box::new(StringColumn::new("region")));
table.add_column(Box::new(StringColumn::new("product")));
table.add_column(Box::new(FloatColumn::new("sales")));
```

#### Adding Data

Add rows to the table:
```
use std::collections::HashMap;

let rows = vec![
    HashMap::from([("region", "North".to_string()), ("product", "A".to_string()), ("sales", "10".to_string())]),
    HashMap::from([("region", "North".to_string()), ("product", "B".to_string()), ("sales", "20".to_string())]),
    HashMap::from([("region", "South".to_string()), ("product", "A".to_string()), ("sales", "5".to_string())]),
];

for row in rows {
    table.push_row_str(&row);
}
```

#### Performing an Inner Join

Join two tables on a key column:
```
use dm::methods::inner_join;

let result = inner_join::inner_join(&left_table, &right_table, "key_column");
```

#### Aggregating Data

Aggregate data using sum or count:
```
use dm::methods::aggregation::{aggregate, AggregationType};
use std::collections::HashMap;

let groups = group_by(&table, "region");
let result = aggregate(&table, &groups, "sales", AggregationType::Sum);
```

#### Creating a Pivot Table
Create a pivot table to summarize data:
```
use dm::methods::pivot;
use dm::methods::aggregation::AggregationType;

let pivoted = pivot::pivot(&table, &["region"], &["product"], "sales", AggregationType::Sum);
```

#### Printing Tables
Print tables in a readable format:
```
println!("Original table:");
println!("{:?}", table);

println!("\nPivoted table:");
println!("{:?}", pivoted);
```

### Contributing
Contributions are welcome! Please open an issue before submitting a pull request on GitHub.

### License
This project is licensed under the MIT License. See the LICENSE file for details.