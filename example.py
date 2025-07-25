import velox

# Create a DataFrame
df = velox.PyDataFrame({
    "a": velox.PySeries([1, 2, 3]),
    "b": velox.PySeries([4, 5, 6])
})

# Print the DataFrame
print(df)

# Group by column 'a' and calculate the sum
grouped_df = df.group_by(["a"])
sum_df = grouped_df.sum()

# Print the aggregated DataFrame
print(sum_df)
