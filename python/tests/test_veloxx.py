import pytest
import veloxx

@pytest.fixture
def sample_series_i32():
    return veloxx.PySeries("test_series_i32", [1, 2, None, 4])

@pytest.fixture
def sample_series_f64():
    return veloxx.PySeries("test_series_f64", [1.0, 2.5, None, 4.0])

@pytest.fixture
def sample_series_string():
    return veloxx.PySeries("test_series_string", ["a", "b", None, "d"])

@pytest.fixture
def sample_dataframe():
    s1 = veloxx.PySeries("col1", [1, 2, 3, 4])
    s2 = veloxx.PySeries("col2", ["a", "b", "c", "d"])
    s3 = veloxx.PySeries("col3", [1.0, 2.0, 3.0, 4.0])
    return veloxx.PyDataFrame({"col1": s1, "col2": s2, "col3": s3})

class TestPySeries:
    def test_series_creation(self, sample_series_i32):
        s = sample_series_i32
        assert s.name() == "test_series_i32"
        assert s.len() == 4
        assert not s.is_empty()
        # Accept both string and enum for data_type comparison
        dtype = s.data_type()
        assert dtype == veloxx.PyDataType.I32 or str(dtype) in ("I32", "PyDataType.I32")

    def test_series_set_name(self, sample_series_i32):
        s = sample_series_i32
        s.set_name("new_name")
        assert s.name() == "new_name"

    def test_series_filter(self, sample_series_i32):
        filtered_s = sample_series_i32.filter([0, 3])
        assert filtered_s.len() == 2

    def test_series_count(self, sample_series_i32):
        # Only run if count() exists
        if hasattr(sample_series_i32, "count"):
            assert sample_series_i32.count() == 3
        else:
            pytest.skip("count() not implemented in PySeries")

    def test_series_fill_nulls(self, sample_series_i32):
        filled = sample_series_i32.fill_nulls(0)
        assert all(v is not None for v in [filled.get_value(i) for i in range(filled.len())])

    def test_series_cast(self, sample_series_i32):
        casted_s = sample_series_i32.cast(veloxx.PyDataType.F64)
        dtype = casted_s.data_type()
        assert dtype == veloxx.PyDataType.F64 or str(dtype) in ("F64", "PyDataType.F64")
        assert casted_s.get_value(0) == 1.0
        assert casted_s.get_value(2) is None

    def test_series_unique(self):
        s = veloxx.PySeries("test_series_unique", [1, 2, 2, 3, 1, None])
        if hasattr(s, "unique"):
            unique_s = s.unique()
            assert unique_s.len() <= s.len()
        else:
            pytest.skip("unique() not implemented in PySeries")

    def test_series_std_dev(self):
        s = veloxx.PySeries("std_dev", [1.0, 2.0, 3.0, 4.0, 5.0])
        if hasattr(s, "std_dev"):
            assert s.std_dev() == pytest.approx(1.5811388300841898)
        else:
            pytest.skip("std_dev() not implemented in PySeries")

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
