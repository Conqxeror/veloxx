import time
import pandas as pd
import polars as pl
import dask.dataframe as dd
from veloxx_python_api import read_csv

def benchmark_pandas():
    start_time = time.time()
    df = pd.read_csv("large_sample.csv")
    filtered_df = df[df['age'] > 30]
    grouped_df = filtered_df.groupby('city').size()
    end_time = time.time()
    return end_time - start_time

def benchmark_polars():
    start_time = time.time()
    df = pl.read_csv("large_sample.csv")
    filtered_df = df.filter(pl.col('age') > 30)
    grouped_df = filtered_df.group_by('city').count()
    end_time = time.time()
    return end_time - start_time

def benchmark_dask():
    start_time = time.time()
    df = dd.read_csv("large_sample.csv")
    filtered_df = df[df['age'] > 30]
    grouped_df = filtered_df.groupby('city').size().compute()
    end_time = time.time()
    return end_time - start_time

def benchmark_veloxx():
    start_time = time.time()
    df = read_csv("large_sample.csv")
    filtered_df = df.filter(lambda row: int(row['age']) > 30)
    grouped_df = filtered_df.group_by('city').count()
    end_time = time.time()
    return end_time - start_time

if __name__ == "__main__":
    print("Benchmarking Pandas...")
    pandas_time = benchmark_pandas()
    print(f"Pandas Time: {pandas_time:.6f} seconds")

    print("Benchmarking Polars...")
    polars_time = benchmark_polars()
    print(f"Polars Time: {polars_time:.6f} seconds")

    print("Benchmarking Dask...")
    dask_time = benchmark_dask()
    print(f"Dask Time: {dask_time:.6f} seconds")

    print("Benchmarking Veloxx...")
    veloxx_time = benchmark_veloxx()
    print(f"Veloxx Time: {veloxx_time:.6f} seconds")