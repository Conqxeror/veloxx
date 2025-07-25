use veloxx::conditions::Condition;
use veloxx::dataframe::join::JoinType;
use veloxx::dataframe::DataFrame;
use veloxx::types::Value;

fn main() -> Result<(), veloxx::error::VeloxxError> {
    let purchases = DataFrame::from_csv("examples/data/purchases.csv")?;
    let users = DataFrame::from_csv("examples/data/users.csv")?;

    let joined_df = purchases.join(&users, "user_id", JoinType::Inner)?;

    let filtered_df = joined_df.filter(&Condition::Gt("age".to_string(), Value::I32(30)))?;

    let result = filtered_df
        .group_by(vec!["product".to_string()])?
        .agg(vec![("price", "sum")])?;

    println!("{}", result);

    Ok(())
}
