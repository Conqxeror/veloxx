import pytest
from veloxx import DataFrame, Series

def test_python_grouped_agg_sum():
    group = [1, 2, 1, 2, 1]
    values = [10.0, 20.0, 30.0, 40.0, 50.0]
    bitmap = [True] * len(group)
    columns = {
        "group": Series.I32("group", group, bitmap),
        "values": Series.F64("values", values, bitmap)
    }
    df = DataFrame(columns)
    grouped = df.group_by(["group"])
    result = grouped.agg([("values", "sum")])
    sums = result["values"].to_list()
    assert sorted(sums) == [90.0, 60.0]
