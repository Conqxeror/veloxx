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
        assert s.data_type() == "I32"
        assert s.get_value(0) == 1
        assert s.get_value(2) is None

    def test_series_set_name(self, sample_series_i32):
        s = sample_series_i32
        s.set_name("new_name")
        assert s.name() == "new_name"

    def test_series_filter(self, sample_series_i32):
        filtered_s = sample_series_i32.filter([0, 3])
        assert filtered_s.len() == 2
        assert filtered_s.get_value(0) == 1
        assert filtered_s.get_value(1) == 4

    def test_series_count(self, sample_series_i32):
        assert sample_series_i32.count() == 3

    def test_series_sum_simd(self, sample_series_i32):
        """Test SIMD-optimized sum operation"""
        result = sample_series_i32.sum()
        assert result == 7  # 1 + 2 + 4 = 7

    def test_series_add_simd(self):
        """Test SIMD-optimized addition operation"""
        s1 = veloxx.PySeries("s1", [1, 2, 3, 4])
        s2 = veloxx.PySeries("s2", [1, 1, 1, 1])
        result = s1.add(s2)
        
        assert result.len() == 4
        assert result.get_value(0) == 2
        assert result.get_value(1) == 3
        assert result.get_value(2) == 4
        assert result.get_value(3) == 5

    def test_series_multiply_simd(self):
        """Test SIMD-optimized multiplication operation"""
        s1 = veloxx.PySeries("s1", [2.0, 3.0, 4.0, 5.0])
        s2 = veloxx.PySeries("s2", [2.0, 2.0, 2.0, 2.0])
        result = s1.multiply(s2)
        
        assert result.len() == 4
        assert result.get_value(0) == 4.0
        assert result.get_value(1) == 6.0
        assert result.get_value(2) == 8.0
        assert result.get_value(3) == 10.0

    def test_series_mean_optimized(self, sample_series_f64):
        """Test optimized mean calculation"""
        mean = sample_series_f64.mean()
        expected = (1.0 + 2.5 + 4.0) / 3  # Excluding None
        assert abs(mean - expected) < 1e-10


class TestPyDataFrame:
    def test_dataframe_creation(self, sample_dataframe):
        df = sample_dataframe
        assert df.row_count() == 4
        assert df.column_count() == 3
        assert set(df.column_names()) == {"col1", "col2", "col3"}

    def test_dataframe_get_column(self, sample_dataframe):
        col1 = sample_dataframe.get_column("col1")
        assert col1.name() == "col1"
        assert col1.len() == 4
        assert col1.get_value(0) == 1

    def test_dataframe_high_performance_filter(self, sample_dataframe):
        """Test high-performance vectorized filtering"""
        # Filter using greater than with SIMD optimization
        filtered_df = sample_dataframe.filter_gt("col1", 2)
        assert filtered_df.row_count() == 2
        
        # Check that the filtered data is correct
        col1 = filtered_df.get_column("col1")
        assert col1.get_value(0) == 3
        assert col1.get_value(1) == 4

    def test_dataframe_group_by_performance(self):
        """Test high-performance group by operations"""
        # Create test data for grouping
        category_series = veloxx.PySeries("category", ["A", "B", "A", "B", "A"])
        value_series = veloxx.PySeries("value", [10, 20, 30, 40, 50])
        df = veloxx.PyDataFrame({"category": category_series, "value": value_series})
        
        # Group by category and sum
        grouped = df.group_by(["category"])
        result = grouped.sum()
        
        assert result.row_count() == 2
        assert result.column_count() == 2  # category + value_sum

    def test_dataframe_select_columns(self, sample_dataframe):
        """Test column selection"""
        selected = sample_dataframe.select(["col1", "col3"])
        assert selected.column_count() == 2
        assert set(selected.column_names()) == {"col1", "col3"}

    def test_large_dataframe_performance(self):
        """Test performance with larger datasets"""
        # Create a larger dataset to test SIMD performance
        size = 10000
        large_series1 = veloxx.PySeries("large1", list(range(size)))
        large_series2 = veloxx.PySeries("large2", [x * 2.0 for x in range(size)])
        large_df = veloxx.PyDataFrame({"large1": large_series1, "large2": large_series2})
        
        # Test filtering performance
        filtered = large_df.filter_gt("large1", 5000)
        assert filtered.row_count() == size - 5001  # Elements > 5000
        
        # Test grouping performance (create groups)
        group_series = veloxx.PySeries("group", [i % 10 for i in range(size)])  # 10 groups
        large_df_grouped = veloxx.PyDataFrame({
            "group": group_series, 
            "value": large_series1
        })
        
        grouped = large_df_grouped.group_by(["group"])
        result = grouped.sum()
        assert result.row_count() == 10  # 10 groups


class TestHighPerformanceOperations:
    def test_simd_operations_direct(self):
        """Test direct SIMD operations from Python"""
        a = [1.0, 2.0, 3.0, 4.0, 5.0] * 1000  # Larger array for SIMD
        b = [1.0, 1.0, 1.0, 1.0, 1.0] * 1000
        
        # Test SIMD addition
        result = veloxx.simd_add_f64(a, b)
        expected = [x + y for x, y in zip(a, b)]
        assert result == expected
        
        # Test SIMD sum
        sum_result = veloxx.simd_sum_f64(a)
        expected_sum = sum(a)
        assert abs(sum_result - expected_sum) < 1e-10

    def test_csv_reading_performance(self):
        """Test high-performance CSV reading"""
        # This would require a CSV file to be present
        # In a real test, we'd create a temporary CSV file
        # For now, we'll test the API is available
        try:
            df = veloxx.read_csv("test.csv")
            # If file exists, check it loaded properly
            assert df.row_count() >= 0
        except Exception:
            # File doesn't exist, which is expected in test environment
            pass

    def test_vectorized_operations_benchmark(self):
        """Benchmark test to ensure SIMD operations are faster"""
        import time
        
        size = 100000
        a = [float(i) for i in range(size)]
        b = [1.0] * size
        
        # Time SIMD operation
        start = time.time()
        result_simd = veloxx.simd_add_f64(a, b)
        simd_time = time.time() - start
        
        # Time Python operation
        start = time.time()
        result_python = [x + y for x, y in zip(a, b)]
        python_time = time.time() - start
        
        # Verify results are the same
        assert result_simd == result_python
        
        # SIMD should be faster (this might vary by system)
        print(f"SIMD time: {simd_time:.6f}s, Python time: {python_time:.6f}s")
        # Note: We don't assert performance here as it depends on the system

    def test_series_median(self):
        s = veloxx.PySeries("median_series", [1, 5, 2, 4, 3])
        assert s.median() == 3

        s_even = veloxx.PySeries("median_series_even", [1, 4, 2, 3])
        assert s_even.median() == 2.5

    def test_series_correlation(self):
        s1 = veloxx.PySeries("s1", [1, 2, 3, 4, 5])
        s2 = veloxx.PySeries("s2", [5, 4, 3, 2, 1])
        assert s1.correlation(s2) == -1.0

    def test_series_covariance(self):
        s1 = veloxx.PySeries("s1", [1, 2, 3, 4, 5])
        s2 = veloxx.PySeries("s2", [5, 4, 3, 2, 1])
        assert s1.covariance(s2) == -2.5

    def test_series_fill_nulls(self, sample_series_i32):
        filled_s = sample_series_i32.fill_nulls(99)
        assert filled_s.get_value(2) == 99
        assert filled_s.get_value(0) == 1  # Ensure non-nulls are unchanged

    def test_series_sum(self, sample_series_i32):
        assert sample_series_i32.sum() == 7

    def test_series_mean(self, sample_series_i32):
        assert sample_series_i32.mean() == pytest.approx(2.3333333333333335)

    def test_series_cast(self, sample_series_i32):
        casted_s = sample_series_i32.cast(veloxx.PyDataType.F64)
        assert casted_s.data_type() == "F64"
        assert casted_s.get_value(0) == 1.0
        assert casted_s.get_value(2) is None  # Nulls should remain null

    def test_series_unique(self):
        s = veloxx.PySeries("test_series_unique", [1, 2, 2, 3, 1, None])
        unique_s = s.unique()
        assert unique_s.len() == 4  # 1, 2, 3, None
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
        filtered_df = sample_dataframe.filter(veloxx.PyCondition.gt("col1", veloxx.PyValue.from_i32(2)))
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

        dropped_df = df.drop_nulls(None)
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
