import pytest
from veloxx import DataFrame, Series
from veloxx.window_functions import WindowFunction, WindowSpec, RankingFunction

def test_window_ranking():
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    bitmap = [True] * len(data)
    columns = {"values": Series.F64("values", data, bitmap)}
    df = DataFrame(columns)
    window_spec = WindowSpec().order_by(["values"])
    result = WindowFunction.apply_ranking(df, RankingFunction.RowNumber, window_spec)
    ranks = result["values_rank"].to_list()
    assert ranks == [1, 2, 3, 4, 5]
