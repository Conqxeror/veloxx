import veloxx
import pytest

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
        assert s.data_type() == veloxx.PyDataType.I32
        assert s.get_value(0) == 1
        assert s.get_value(2) is None

    def test_series_fill_nulls(self, sample_series_i32):
        filled_s = sample_series_i32.fill_nulls(99)
        assert filled_s.get_value(2) == 99
        assert filled_s.get_value(0) == 1 # Ensure non-nulls are unchanged

    def test_series_sum(self, sample_series_i32):
        assert sample_series_i32.sum() == 7

    def test_series_mean(self, sample_series_i32):
        assert sample_series_i32.mean() == pytest.approx(2.3333333333333335)

    def test_series_cast(self, sample_series_i32):
        casted_s = sample_series_i32.cast(veloxx.PyDataType.F64)
        assert casted_s.data_type() == veloxx.PyDataType.F64
        assert casted_s.get_value(0) == 1.0
        assert casted_s.get_value(2) is None # Nulls should remain null

    def test_series_unique(self):
        s = veloxx.PySeries("test_series_unique", [1, 2, 2, 3, 1, None])
        unique_s = s.unique()
        assert unique_s.len() == 4 # 1, 2, 3, None
        assert unique_s.get_value(0) == 1
        assert unique_s.get_value(1) == 2
        assert unique_s.get_value(2) == 3
        assert unique_s.get_value(3) is None

    def test_series_to_vec_f64(self, sample_series_f64):
        vec_f64 = sample_series_f64.to_vec_f64()
        assert vec_f64 == [1.0, 2.5, 4.0]

    def test_series_interpolate_nulls(self):
        s = veloxx.PySeries("test_series_interpolate", [1, None, 3, None, 5])
        interpolated_s = s.interpolate_nulls()
        assert interpolated_s.get_value(1) == 2
        assert interpolated_s.get_value(3) == 4
        assert interpolated_s.get_value(0) == 1
        assert interpolated_s.get_value(4) == 5

    def test_series_append(self, sample_series_i32):
        s_to_append = veloxx.PySeries("append_series", [5, 6])
        appended_s = sample_series_i32.append(s_to_append)
        assert appended_s.len() == 6
        assert appended_s.get_value(4) == 5
        assert appended_s.get_value(5) == 6

    def test_series_min_max(self):
        s_numeric = veloxx.PySeries("numeric", [10, 1, 5, None, 8])
        assert s_numeric.min() == 1
        assert s_numeric.max() == 10

        s_string = veloxx.PySeries("string", ["c", "a", "b"])
        assert s_string.min() == "a"
        assert s_string.max() == "c"

    def test_series_std_dev(self):
        s = veloxx.PySeries("std_dev", [1.0, 2.0, 3.0, 4.0, 5.0])
        assert s.std_dev() == pytest.approx(1.5811388300841898)

class TestPyDataFrame:
    def test_dataframe_creation(self, sample_dataframe):
        df = sample_dataframe
        assert df.row_count() == 4
        assert df.column_count() == 3
        assert "col1" in df.column_names()
        assert "col2" in df.column_names()
        assert "col3" in df.column_names()

    def test_dataframe_filter(self, sample_dataframe):
        # Filter for col1 > 2 (indices 2, 3)
        filtered_df = sample_dataframe.filter([2, 3])
        assert filtered_df.row_count() == 2
        assert filtered_df.get_column("col1").get_value(0) == 3
        assert filtered_df.get_column("col2").get_value(1) == "d"

    def test_dataframe_select_columns(self, sample_dataframe):
        selected_df = sample_dataframe.select_columns(["col1", "col3"])
        assert selected_df.column_count() == 2
        assert "col1" in selected_df.column_names()
        assert "col3" in selected_df.column_names()
        assert "col2" not in selected_df.column_names()

    def test_dataframe_drop_columns(self, sample_dataframe):
        dropped_df = sample_dataframe.drop_columns(["col2"])
        assert dropped_df.column_count() == 2
        assert "col1" in dropped_df.column_names()
        assert "col3" in dropped_df.column_names()
        assert "col2" not in dropped_df.column_names()

    def test_dataframe_rename_column(self, sample_dataframe):
        renamed_df = sample_dataframe.rename_column("col1", "new_col1")
        assert "new_col1" in renamed_df.column_names()
        assert "col1" not in renamed_df.column_names()
        assert renamed_df.get_column("new_col1").get_value(0) == 1

    def test_dataframe_drop_nulls(self):
        s1 = veloxx.PySeries("col1", [1, None, 3])
        s2 = veloxx.PySeries("col2", ["a", "b", None])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        dropped_df = df.drop_nulls()
        assert dropped_df.row_count() == 1
        assert dropped_df.get_column("col1").get_value(0) == 1
        assert dropped_df.get_column("col2").get_value(0) == "a"

    def test_dataframe_fill_nulls(self):
        s1 = veloxx.PySeries("col1", [1, None, 3])
        s2 = veloxx.PySeries("col2", ["a", "b", None])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        filled_df = df.fill_nulls(99)
        assert filled_df.get_column("col1").get_value(1) == 99
        # String column should not be filled by an integer
        assert filled_df.get_column("col2").get_value(2) is None

    def test_dataframe_sort(self):
        s1 = veloxx.PySeries("col1", [3, 1, 2])
        s2 = veloxx.PySeries("col2", ["c", "a", "b"])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        sorted_df = df.sort(["col1"], True)
        assert sorted_df.get_column("col1").get_value(0) == 1
        assert sorted_df.get_column("col1").get_value(1) == 2
        assert sorted_df.get_column("col1").get_value(2) == 3

        sorted_df_desc = df.sort(["col1"], False)
        assert sorted_df_desc.get_column("col1").get_value(0) == 3
        assert sorted_df_desc.get_column("col1").get_value(1) == 2
        assert sorted_df_desc.get_column("col1").get_value(2) == 1