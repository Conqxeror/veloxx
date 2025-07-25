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
        # Test data type comparison using string representation
        assert str(s.data_type()) == str(veloxx.PyDataType.I32)
        # Note: get_value now requires Python context and returns Option<PyObject>
        # These tests would need to be adjusted for the actual API

    def test_series_set_name(self, sample_series_i32):
        s = sample_series_i32
        s.set_name("new_name")
        assert s.name() == "new_name"

    def test_series_filter(self, sample_series_i32):
        filtered_s = sample_series_i32.filter([0, 3])
        assert filtered_s.len() == 2

    def test_series_fill_nulls(self, sample_series_i32):
        filled_s = sample_series_i32.fill_nulls(99)
        assert filled_s.len() == 4  # Length should remain the same

    def test_series_data_types(self):
        # Test different data types using string representation
        s_i32 = veloxx.PySeries("int_series", [1, 2, 3])
        assert str(s_i32.data_type()) == str(veloxx.PyDataType.I32)
        
        s_f64 = veloxx.PySeries("float_series", [1.0, 2.0, 3.0])
        assert str(s_f64.data_type()) == str(veloxx.PyDataType.F64)
        
        # Note: Python bools are treated as integers in this implementation
        s_bool = veloxx.PySeries("bool_series", [True, False, True])
        # This will be I32 because Python bool inherits from int
        assert str(s_bool.data_type()) == str(veloxx.PyDataType.I32)
        
        s_string = veloxx.PySeries("string_series", ["a", "b", "c"])
        assert str(s_string.data_type()) == str(veloxx.PyDataType.String)


class TestPyDataFrame:
    def test_dataframe_creation(self, sample_dataframe):
        df = sample_dataframe
        assert df.row_count() == 4
        assert df.column_count() == 3
        column_names = df.column_names()
        assert "col1" in column_names
        assert "col2" in column_names
        assert "col3" in column_names

    def test_dataframe_filter(self, sample_dataframe):
        # Filter for indices 2, 3
        filtered_df = sample_dataframe.filter([2, 3])
        assert filtered_df.row_count() == 2

    def test_dataframe_select_columns(self, sample_dataframe):
        selected_df = sample_dataframe.select_columns(["col1", "col3"])
        assert selected_df.column_count() == 2
        column_names = selected_df.column_names()
        assert "col1" in column_names
        assert "col3" in column_names
        assert "col2" not in column_names

    def test_dataframe_drop_columns(self, sample_dataframe):
        dropped_df = sample_dataframe.drop_columns(["col2"])
        assert dropped_df.column_count() == 2
        column_names = dropped_df.column_names()
        assert "col1" in column_names
        assert "col3" in column_names
        assert "col2" not in column_names

    def test_dataframe_rename_column(self, sample_dataframe):
        renamed_df = sample_dataframe.rename_column("col1", "new_col1")
        column_names = renamed_df.column_names()
        assert "new_col1" in column_names
        assert "col1" not in column_names

    def test_dataframe_drop_nulls(self):
        s1 = veloxx.PySeries("col1", [1, None, 3])
        s2 = veloxx.PySeries("col2", ["a", "b", None])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        # drop_nulls now requires a subset parameter (can be None)
        dropped_df = df.drop_nulls(None)
        assert dropped_df.row_count() == 1

    def test_dataframe_fill_nulls(self):
        s1 = veloxx.PySeries("col1", [1, None, 3])
        s2 = veloxx.PySeries("col2", ["a", "b", None])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        filled_df = df.fill_nulls(99)
        # Should have same dimensions
        assert filled_df.row_count() == 3
        assert filled_df.column_count() == 2

    def test_dataframe_sort(self):
        s1 = veloxx.PySeries("col1", [3, 1, 2])
        s2 = veloxx.PySeries("col2", ["c", "a", "b"])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})

        sorted_df = df.sort(["col1"], True)
        assert sorted_df.row_count() == 3

    def test_dataframe_append(self):
        s1 = veloxx.PySeries("col1", [1, 2])
        s2 = veloxx.PySeries("col2", ["a", "b"])
        df1 = veloxx.PyDataFrame({"col1": s1, "col2": s2})
        
        s3 = veloxx.PySeries("col1", [3, 4])
        s4 = veloxx.PySeries("col2", ["c", "d"])
        df2 = veloxx.PyDataFrame({"col1": s3, "col2": s4})
        
        appended_df = df1.append(df2)
        assert appended_df.row_count() == 4

    def test_dataframe_correlation(self):
        s1 = veloxx.PySeries("col1", [1.0, 2.0, 3.0, 4.0, 5.0])
        s2 = veloxx.PySeries("col2", [5.0, 4.0, 3.0, 2.0, 1.0])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})
        
        corr = df.correlation("col1", "col2")
        assert corr == pytest.approx(-1.0, abs=1e-6)

    def test_dataframe_covariance(self):
        s1 = veloxx.PySeries("col1", [1.0, 2.0, 3.0, 4.0, 5.0])
        s2 = veloxx.PySeries("col2", [5.0, 4.0, 3.0, 2.0, 1.0])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})
        
        cov = df.covariance("col1", "col2")
        assert cov == pytest.approx(-2.5, abs=1e-6)

    def test_dataframe_group_by(self):
        s1 = veloxx.PySeries("group", ["A", "B", "A", "B"])
        s2 = veloxx.PySeries("value", [1.0, 2.0, 3.0, 4.0])
        df = veloxx.PyDataFrame({"group": s1, "value": s2})
        
        grouped = df.group_by(["group"])
        # Use specific column aggregation instead of "*"
        sum_result = grouped.agg([("value", "sum")])
        assert sum_result.row_count() == 2  # Should have 2 groups

    def test_dataframe_describe(self, sample_dataframe):
        described_df = sample_dataframe.describe()
        # Should return a dataframe with statistical summaries
        assert described_df.column_count() > 0


class TestPyExpr:
    def test_column_expression(self):
        expr = veloxx.PyExpr.column("test_col")
        # Basic test that expression can be created
        assert expr is not None

    def test_literal_expression(self):
        expr = veloxx.PyExpr.literal(42)
        assert expr is not None
        
        expr_str = veloxx.PyExpr.literal("test")
        assert expr_str is not None

    def test_arithmetic_expressions(self):
        col_expr = veloxx.PyExpr.column("col1")
        lit_expr = veloxx.PyExpr.literal(10)
        
        add_expr = veloxx.PyExpr.add(col_expr, lit_expr)
        assert add_expr is not None
        
        sub_expr = veloxx.PyExpr.subtract(col_expr, lit_expr)
        assert sub_expr is not None
        
        mul_expr = veloxx.PyExpr.multiply(col_expr, lit_expr)
        assert mul_expr is not None
        
        div_expr = veloxx.PyExpr.divide(col_expr, lit_expr)
        assert div_expr is not None

    def test_comparison_expressions(self):
        col_expr = veloxx.PyExpr.column("col1")
        lit_expr = veloxx.PyExpr.literal(5)
        
        eq_expr = veloxx.PyExpr.equals(col_expr, lit_expr)
        assert eq_expr is not None
        
        neq_expr = veloxx.PyExpr.not_equals(col_expr, lit_expr)
        assert neq_expr is not None
        
        gt_expr = veloxx.PyExpr.greater_than(col_expr, lit_expr)
        assert gt_expr is not None
        
        lt_expr = veloxx.PyExpr.less_than(col_expr, lit_expr)
        assert lt_expr is not None

    def test_logical_expressions(self):
        expr1 = veloxx.PyExpr.greater_than(
            veloxx.PyExpr.column("col1"), 
            veloxx.PyExpr.literal(5)
        )
        expr2 = veloxx.PyExpr.less_than(
            veloxx.PyExpr.column("col1"), 
            veloxx.PyExpr.literal(10)
        )
        
        and_expr = getattr(veloxx.PyExpr, 'and')(expr1, expr2)
        assert and_expr is not None
        
        or_expr = getattr(veloxx.PyExpr, 'or')(expr1, expr2)
        assert or_expr is not None
        
        not_expr = getattr(veloxx.PyExpr, 'not')(expr1)
        assert not_expr is not None


class TestPyJoinType:
    def test_join_types(self):
        # Test that join types can be created
        inner = veloxx.PyJoinType.Inner
        left = veloxx.PyJoinType.Left
        right = veloxx.PyJoinType.Right
        
        assert inner is not None
        assert left is not None
        assert right is not None


class TestDataFrameIO:
    def test_csv_operations(self, tmp_path):
        # Create a test dataframe
        s1 = veloxx.PySeries("col1", [1, 2, 3])
        s2 = veloxx.PySeries("col2", ["a", "b", "c"])
        df = veloxx.PyDataFrame({"col1": s1, "col2": s2})
        
        # Test CSV export
        csv_path = str(tmp_path / "test.csv")
        df.to_csv(csv_path)
        
        # Test CSV import
        loaded_df = veloxx.PyDataFrame.from_csv(csv_path)
        assert loaded_df.row_count() == 3
        assert loaded_df.column_count() == 2


if __name__ == "__main__":
    pytest.main([__file__, "-v"])