import veloxx
import pytest

def test_basic_import():
    """Test that we can import veloxx successfully"""
    assert hasattr(veloxx, 'PySeries')
    assert hasattr(veloxx, 'PyDataFrame')

def test_series_creation():
    """Test basic series creation"""
    s = veloxx.PySeries("test_series", [1, 2, 3, 4])
    assert s.name() == "test_series"
    assert s.len() == 4
    assert s.get_value(0) == 1
    assert s.get_value(3) == 4

def test_series_with_nulls():
    """Test series creation with None values"""
    s = veloxx.PySeries("test_nulls", [1, None, 3, None])
    assert s.len() == 4
    assert s.get_value(0) == 1
    assert s.get_value(1) is None
    assert s.get_value(2) == 3
    assert s.get_value(3) is None

def test_dataframe_creation_simple():
    """Test simple DataFrame creation with matching names"""
    s1 = veloxx.PySeries("col1", [1, 2, 3])
    s2 = veloxx.PySeries("col2", [4, 5, 6])
    
    # Create DataFrame with series that have matching names
    df = veloxx.PyDataFrame({"col1": s1, "col2": s2})
    assert df.row_count() == 3
    assert df.column_count() == 2
    assert "col1" in df.column_names()
    assert "col2" in df.column_names()

if __name__ == "__main__":
    test_basic_import()
    test_series_creation()
    test_series_with_nulls()
    test_dataframe_creation_simple()
    print("All basic tests passed!")