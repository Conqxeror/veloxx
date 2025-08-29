import pytest
from veloxx import Series

def test_quantile_i32():
    data = [1, 3, 2, 4, 5]
    bitmap = [True] * len(data)
    s = Series.I32("test", data, bitmap)
    assert s.quantile(0.5) == 3
    assert s.quantile(0.0) == 1
    assert s.quantile(1.0) == 5

def test_quantile_f64():
    data = [1.0, 2.0, 3.0, 4.0]
    bitmap = [True] * len(data)
    s = Series.F64("test", data, bitmap)
    assert s.quantile(0.5) == 2.5
    assert s.quantile(0.0) == 1.0
    assert s.quantile(1.0) == 4.0
