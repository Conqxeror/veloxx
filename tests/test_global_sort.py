import pytest
from veloxx import GlobalSort

def test_global_sort_f64():
    data = [3.0, 1.0, 4.0, 2.0]
    GlobalSort.sort_f64(data)
    assert data == [1.0, 2.0, 3.0, 4.0]
