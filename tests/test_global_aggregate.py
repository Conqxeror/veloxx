import pytest
from veloxx import GlobalAggregate

def test_global_sum_mean_f64():
    data = [1.0, 2.0, 3.0, 4.0]
    assert GlobalAggregate.sum_f64(data) == 10.0
    assert GlobalAggregate.mean_f64(data) == 2.5
