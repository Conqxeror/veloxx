import pytest
from veloxx import Series

def test_median_i32():
    data = [1, 3, 2, 4, 5]
    bitmap = [True] * len(data)
    s = Series.I32("test", data, bitmap)
    assert s.median() == 3

def test_median_f64():
    data = [1.0, 2.0, 3.0, 4.0]
    bitmap = [True] * len(data)
    s = Series.F64("test", data, bitmap)
    assert s.median() == 2.5
