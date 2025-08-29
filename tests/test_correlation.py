import pytest
from veloxx import Series

def test_correlation():
    data1 = [1.0, 2.0, 3.0, 4.0, 5.0]
    data2 = [2.0, 4.0, 6.0, 8.0, 10.0]
    bitmap = [True] * len(data1)
    s1 = Series.F64("s1", data1, bitmap)
    s2 = Series.F64("s2", data2, bitmap)
    corr = s1.correlation(s2)
    assert abs(corr - 1.0) < 1e-6
