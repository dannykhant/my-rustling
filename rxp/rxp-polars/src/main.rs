use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = df![
        "Name" => &["Alice", "Bob", "Charlie", "David", "Eve"],
        "Age" => &[25, 30, 31, 21, 35],
        "City" => &["New York", "Los Angeles", "Chicago", "Houston", "Chicago"]
    ]?;

    let lf = df.lazy();

    let avg_age = &lf.group_by(["City"])
        .agg([col("Age").mean().alias("Average_Age")])
        .collect();

    println!("Average age: {:?}", avg_age);

    Ok(())
}
