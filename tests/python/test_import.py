import veloxx

try:
    s = veloxx.PySeries("test", [1, 2, 3])
    print(f"PySeries created: {s.name()}")
    df = veloxx.PyDataFrame({"col1": s})
    print(f"PyDataFrame created: {df.row_count()} rows")
except AttributeError as e:
    print(f"AttributeError: {e}")
except Exception as e:
    print(f"An unexpected error occurred: {e}")