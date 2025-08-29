import pytest
from veloxx import Series

def test_percentile_i32():
    data = [1, 3, 2, 4, 5]
    bitmap = [True] * len(data)
    s = Series.I32("test", data, bitmap)
    assert s.percentile(50.0) == 3
    assert s.percentile(0.0) == 1
    assert s.percentile(100.0) == 5

def test_percentile_f64():
    data = [1.0, 2.0, 3.0, 4.0]
    bitmap = [True] * len(data)
    s = Series.F64("test", data, bitmap)
    assert s.percentile(50.0) == 2.5
    assert s.percentile(0.0) == 1.0
    assert s.percentile(100.0) == 4.0
