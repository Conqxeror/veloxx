# Tutorial: Customer Purchase Analysis

This tutorial demonstrates a more advanced workflow with the Veloxx library, including reading data from multiple CSV files, joining DataFrames, and performing a group-by aggregation.

## The Data

We will be working with two CSV files:

*   `users.csv`: Contains information about our users.
*   `purchases.csv`: Contains information about the purchases made by our users.

## The Goal

Our goal is to calculate the total amount of money spent by each user.

## The Code

```rust
use veloxx::dataframe::DataFrame;
use veloxx::prelude::*;

fn main() -> Result<(), VeloxxError> {
    // 1. Read the data from the CSV files.
    let users_df = DataFrame::from_csv("examples/data/users.csv")?;
    let purchases_df = DataFrame::from_csv("examples/data/purchases.csv")?;

    // 2. Join the two DataFrames.
    let df = users_df.join(&purchases_df, "user_id", JoinType::Inner)?;

    // 3. Group the data by user and calculate the total price of their purchases.
    let grouped_df = df.group_by(vec!["user_id".to_string(), "name".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![("price", "sum")])?;

    // 4. Print the result.
    println!("{}", aggregated_df);

    Ok(())
}
```

## The Explanation

1.  We start by reading the `users.csv` and `purchases.csv` files into two separate DataFrames.
2.  We then join the two DataFrames on the `user_id` column. We use an inner join to ensure that we only keep the users who have made purchases.
3.  We then group the data by `user_id` and `name` and calculate the sum of the `price` column for each group.
4.  Finally, we print the result to the console.
