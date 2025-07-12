Advanced Features
=================

This section covers advanced features and techniques for power users of Veloxx.

Performance Optimization
------------------------

Understanding Veloxx's Architecture
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Veloxx is built with a Rust core that provides several performance advantages:

* **Zero-copy operations**: Many operations avoid unnecessary data copying
* **Memory safety**: Rust's ownership system prevents memory leaks and segfaults
* **SIMD optimizations**: Vectorized operations for numeric computations
* **Efficient memory layout**: Columnar storage for better cache locality

Benchmarking Your Operations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    import time
    import veloxx

    def benchmark_operation(operation_func, data, iterations=100):
        """Benchmark a Veloxx operation"""
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            result = operation_func(data)
            end = time.perf_counter()
            times.append(end - start)
        
        avg_time = sum(times) / len(times)
        min_time = min(times)
        max_time = max(times)
        
        return {
            "average": avg_time,
            "min": min_time,
            "max": max_time,
            "total": sum(times)
        }

    # Example: Benchmark filtering operation
    def filter_operation(df):
        sales = df.get_column("sales").to_vec_f64()
        indices = [i for i, sale in enumerate(sales) if sale > 500]
        return df.filter(indices)

    # Create test data
    test_df = veloxx.PyDataFrame({
        "id": veloxx.PySeries("id", list(range(10000))),
        "sales": veloxx.PySeries("sales", [i * 0.1 for i in range(10000)])
    })

    # Benchmark the operation
    results = benchmark_operation(filter_operation, test_df, iterations=50)
    print(f"Filter operation benchmark:")
    print(f"Average time: {results['average']*1000:.2f}ms")
    print(f"Min time: {results['min']*1000:.2f}ms")
    print(f"Max time: {results['max']*1000:.2f}ms")

Memory-Efficient Processing
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def process_large_dataset_efficiently(df, chunk_size=1000):
        """Process large datasets in chunks to manage memory"""
        total_rows = df.shape()[0]
        results = []
        
        for start_idx in range(0, total_rows, chunk_size):
            end_idx = min(start_idx + chunk_size, total_rows)
            chunk_indices = list(range(start_idx, end_idx))
            
            # Process chunk
            chunk = df.filter(chunk_indices)
            
            # Perform operations on chunk
            sales = chunk.get_column("sales").to_vec_f64()
            chunk_total = sum(sales)
            chunk_avg = chunk_total / len(sales)
            
            results.append({
                "chunk": start_idx // chunk_size,
                "rows": len(chunk_indices),
                "total_sales": chunk_total,
                "avg_sales": chunk_avg
            })
        
        return results

    # Example usage
    large_df = veloxx.PyDataFrame({
        "id": veloxx.PySeries("id", list(range(5000))),
        "sales": veloxx.PySeries("sales", [i * 0.5 for i in range(5000)])
    })

    chunk_results = process_large_dataset_efficiently(large_df, chunk_size=1000)
    print("Chunk processing results:")
    for result in chunk_results:
        print(f"Chunk {result['chunk']}: {result['rows']} rows, "
              f"Total: ${result['total_sales']:,.2f}, "
              f"Avg: ${result['avg_sales']:.2f}")

Advanced Data Manipulation
--------------------------

Complex Filtering Strategies
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def multi_condition_filter(df, conditions):
        """Apply multiple filtering conditions efficiently"""
        # Get all data once
        columns_data = {}
        for col_name in df.columns():
            series = df.get_column(col_name)
            try:
                columns_data[col_name] = series.to_vec_f64()
            except:
                try:
                    columns_data[col_name] = series.to_vec_string()
                except:
                    columns_data[col_name] = series.to_vec_bool()
        
        # Apply all conditions
        valid_indices = []
        num_rows = df.shape()[0]
        
        for i in range(num_rows):
            row_data = {col: data[i] for col, data in columns_data.items()}
            
            # Check all conditions
            if all(condition(row_data) for condition in conditions):
                valid_indices.append(i)
        
        return df.filter(valid_indices)

    # Example: Complex employee filtering
    employee_df = veloxx.PyDataFrame({
        "name": veloxx.PySeries("name", [
            "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank"
        ]),
        "age": veloxx.PySeries("age", [28, 35, 42, 29, 38, 45]),
        "salary": veloxx.PySeries("salary", [
            75000, 85000, 95000, 70000, 90000, 100000
        ]),
        "department": veloxx.PySeries("department", [
            "Engineering", "Sales", "Engineering", "HR", "Sales", "Engineering"
        ]),
        "years_experience": veloxx.PySeries("years_experience", [5, 10, 15, 3, 12, 20])
    })

    # Define complex conditions
    conditions = [
        lambda row: row["age"] >= 30,  # Age 30 or older
        lambda row: row["salary"] >= 80000,  # Salary 80k or more
        lambda row: row["years_experience"] >= 8,  # 8+ years experience
        lambda row: row["department"] in ["Engineering", "Sales"]  # Specific departments
    ]

    # Apply complex filter
    filtered_employees = multi_condition_filter(employee_df, conditions)
    print("Senior employees meeting all criteria:")
    print(filtered_employees)

Custom Aggregation Functions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def custom_aggregation(df, group_column, agg_column, agg_functions):
        """Perform custom aggregations on grouped data"""
        # Get grouping and aggregation data
        group_data = df.get_column(group_column).to_vec_string()
        
        # Determine aggregation data type
        agg_series = df.get_column(agg_column)
        try:
            agg_data = agg_series.to_vec_f64()
            is_numeric = True
        except:
            agg_data = agg_series.to_vec_string()
            is_numeric = False
        
        # Group data
        groups = {}
        for i, (group_val, agg_val) in enumerate(zip(group_data, agg_data)):
            if group_val not in groups:
                groups[group_val] = []
            groups[group_val].append(agg_val)
        
        # Apply aggregation functions
        results = {}
        for group_name, values in groups.items():
            group_results = {}
            for func_name, func in agg_functions.items():
                try:
                    group_results[func_name] = func(values)
                except Exception as e:
                    group_results[func_name] = f"Error: {e}"
            results[group_name] = group_results
        
        return results

    # Define custom aggregation functions
    numeric_agg_functions = {
        "sum": sum,
        "mean": lambda x: sum(x) / len(x),
        "min": min,
        "max": max,
        "range": lambda x: max(x) - min(x),
        "count": len,
        "std_dev": lambda x: (sum((val - sum(x)/len(x))**2 for val in x) / len(x))**0.5
    }

    # Apply custom aggregations
    salary_by_dept = custom_aggregation(
        employee_df, "department", "salary", numeric_agg_functions
    )

    print("Salary statistics by department:")
    for dept, stats in salary_by_dept.items():
        print(f"\n{dept}:")
        for stat_name, value in stats.items():
            if isinstance(value, float):
                print(f"  {stat_name}: {value:,.2f}")
            else:
                print(f"  {stat_name}: {value}")

Data Pipeline Construction
--------------------------

Building Reusable Data Pipelines
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    class VeloxxPipeline:
        """A reusable data processing pipeline for Veloxx DataFrames"""
        
        def __init__(self):
            self.steps = []
            self.metadata = {}
        
        def add_step(self, name, function, **kwargs):
            """Add a processing step to the pipeline"""
            self.steps.append({
                "name": name,
                "function": function,
                "kwargs": kwargs
            })
            return self
        
        def execute(self, df):
            """Execute the pipeline on a DataFrame"""
            current_df = df
            execution_log = []
            
            for step in self.steps:
                start_time = time.perf_counter()
                start_shape = current_df.shape()
                
                try:
                    current_df = step["function"](current_df, **step["kwargs"])
                    end_time = time.perf_counter()
                    end_shape = current_df.shape()
                    
                    execution_log.append({
                        "step": step["name"],
                        "status": "success",
                        "duration": end_time - start_time,
                        "input_shape": start_shape,
                        "output_shape": end_shape
                    })
                except Exception as e:
                    execution_log.append({
                        "step": step["name"],
                        "status": "error",
                        "error": str(e),
                        "duration": time.perf_counter() - start_time
                    })
                    break
            
            return current_df, execution_log

    # Define pipeline steps
    def filter_high_performers(df, min_score=90):
        """Filter for high-performing employees"""
        scores = df.get_column("performance_score").to_vec_f64()
        indices = [i for i, score in enumerate(scores) if score >= min_score]
        return df.filter(indices)

    def add_bonus_eligibility(df, min_experience=5):
        """Add bonus eligibility based on experience"""
        experience = df.get_column("years_experience").to_vec_f64()
        eligibility = [exp >= min_experience for exp in experience]
        
        # Create new DataFrame with bonus eligibility
        new_df_data = {}
        for col_name in df.columns():
            new_df_data[col_name] = df.get_column(col_name)
        new_df_data["bonus_eligible"] = veloxx.PySeries("bonus_eligible", eligibility)
        
        return veloxx.PyDataFrame(new_df_data)

    def select_key_columns(df, columns=None):
        """Select only key columns for final output"""
        if columns is None:
            columns = ["name", "department", "performance_score", "bonus_eligible"]
        return df.select_columns(columns)

    # Build and execute pipeline
    pipeline = VeloxxPipeline()
    pipeline.add_step("filter_performers", filter_high_performers, min_score=85)
    pipeline.add_step("add_bonus", add_bonus_eligibility, min_experience=3)
    pipeline.add_step("select_columns", select_key_columns)

    # Execute on employee data
    result_df, log = pipeline.execute(employee_df)

    print("Pipeline execution results:")
    print(result_df)
    
    print("\nExecution log:")
    for entry in log:
        if entry["status"] == "success":
            print(f"✓ {entry['step']}: {entry['input_shape']} → {entry['output_shape']} "
                  f"({entry['duration']*1000:.2f}ms)")
        else:
            print(f"✗ {entry['step']}: Error - {entry['error']}")

Advanced Type Handling
----------------------

Working with Mixed Data Types
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def analyze_data_types(df):
        """Analyze and report data types in a DataFrame"""
        type_info = {}
        
        for col_name in df.columns():
            series = df.get_column(col_name)
            
            # Try to determine the actual data type
            try:
                data = series.to_vec_f64()
                type_info[col_name] = {
                    "type": "numeric",
                    "sample": data[:3],
                    "stats": {
                        "min": min(data),
                        "max": max(data),
                        "mean": sum(data) / len(data)
                    }
                }
            except:
                try:
                    data = series.to_vec_string()
                    unique_values = list(set(data))
                    type_info[col_name] = {
                        "type": "string",
                        "sample": data[:3],
                        "stats": {
                            "unique_count": len(unique_values),
                            "most_common": max(unique_values, key=data.count)
                        }
                    }
                except:
                    try:
                        data = series.to_vec_bool()
                        type_info[col_name] = {
                            "type": "boolean",
                            "sample": data[:3],
                            "stats": {
                                "true_count": sum(data),
                                "false_count": len(data) - sum(data)
                            }
                        }
                    except:
                        type_info[col_name] = {
                            "type": "unknown",
                            "sample": "N/A",
                            "stats": {}
                        }
        
        return type_info

    # Analyze employee DataFrame
    type_analysis = analyze_data_types(employee_df)
    
    print("Data type analysis:")
    for col_name, info in type_analysis.items():
        print(f"\n{col_name}:")
        print(f"  Type: {info['type']}")
        print(f"  Sample: {info['sample']}")
        for stat_name, stat_value in info['stats'].items():
            print(f"  {stat_name}: {stat_value}")

Custom Data Validation
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def validate_dataframe(df, validation_rules):
        """Validate DataFrame against custom rules"""
        validation_results = []
        
        for rule_name, rule_func in validation_rules.items():
            try:
                result = rule_func(df)
                validation_results.append({
                    "rule": rule_name,
                    "status": "passed" if result["valid"] else "failed",
                    "message": result["message"],
                    "details": result.get("details", {})
                })
            except Exception as e:
                validation_results.append({
                    "rule": rule_name,
                    "status": "error",
                    "message": f"Validation error: {str(e)}",
                    "details": {}
                })
        
        return validation_results

    # Define validation rules
    def validate_salary_range(df):
        """Validate that salaries are within reasonable range"""
        salaries = df.get_column("salary").to_vec_f64()
        min_salary = min(salaries)
        max_salary = max(salaries)
        
        valid = 20000 <= min_salary and max_salary <= 500000
        return {
            "valid": valid,
            "message": f"Salary range: ${min_salary:,.2f} - ${max_salary:,.2f}",
            "details": {"min": min_salary, "max": max_salary}
        }

    def validate_age_consistency(df):
        """Validate age and experience consistency"""
        ages = df.get_column("age").to_vec_f64()
        experience = df.get_column("years_experience").to_vec_f64()
        
        inconsistencies = []
        for i, (age, exp) in enumerate(zip(ages, experience)):
            if exp > age - 16:  # Assuming minimum working age of 16
                inconsistencies.append(i)
        
        valid = len(inconsistencies) == 0
        return {
            "valid": valid,
            "message": f"Found {len(inconsistencies)} age/experience inconsistencies",
            "details": {"inconsistent_rows": inconsistencies}
        }

    def validate_required_columns(df):
        """Validate that required columns exist"""
        required_cols = ["name", "age", "salary", "department"]
        existing_cols = df.columns()
        missing_cols = [col for col in required_cols if col not in existing_cols]
        
        valid = len(missing_cols) == 0
        return {
            "valid": valid,
            "message": f"Missing columns: {missing_cols}" if missing_cols else "All required columns present",
            "details": {"missing": missing_cols, "existing": existing_cols}
        }

    # Run validation
    validation_rules = {
        "salary_range": validate_salary_range,
        "age_consistency": validate_age_consistency,
        "required_columns": validate_required_columns
    }

    validation_results = validate_dataframe(employee_df, validation_rules)

    print("Data validation results:")
    for result in validation_results:
        status_symbol = "✓" if result["status"] == "passed" else "✗"
        print(f"{status_symbol} {result['rule']}: {result['message']}")
        if result["details"]:
            print(f"   Details: {result['details']}")

Integration with External Systems
---------------------------------

Database-like Operations
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def join_dataframes(left_df, right_df, left_key, right_key, join_type="inner"):
        """Perform join operations between two DataFrames"""
        # Get join keys
        left_keys = left_df.get_column(left_key).to_vec_string()
        right_keys = right_df.get_column(right_key).to_vec_string()
        
        # Create lookup for right DataFrame
        right_lookup = {}
        for i, key in enumerate(right_keys):
            if key not in right_lookup:
                right_lookup[key] = []
            right_lookup[key].append(i)
        
        # Perform join
        result_indices = {"left": [], "right": []}
        
        for left_idx, left_key_val in enumerate(left_keys):
            if left_key_val in right_lookup:
                # Match found
                for right_idx in right_lookup[left_key_val]:
                    result_indices["left"].append(left_idx)
                    result_indices["right"].append(right_idx)
            elif join_type == "left":
                # Left join: include left row even without match
                result_indices["left"].append(left_idx)
                result_indices["right"].append(None)
        
        # Handle right join case
        if join_type == "right":
            for right_idx, right_key_val in enumerate(right_keys):
                if right_key_val not in left_keys:
                    result_indices["left"].append(None)
                    result_indices["right"].append(right_idx)
        
        # Build result DataFrame
        result_data = {}
        
        # Add left DataFrame columns
        for col_name in left_df.columns():
            if col_name != left_key:  # Avoid duplicate key column
                left_series = left_df.get_column(col_name)
                try:
                    left_data = left_series.to_vec_f64()
                    result_values = [left_data[i] if i is not None else 0.0 
                                   for i in result_indices["left"]]
                except:
                    try:
                        left_data = left_series.to_vec_string()
                        result_values = [left_data[i] if i is not None else "" 
                                       for i in result_indices["left"]]
                    except:
                        left_data = left_series.to_vec_bool()
                        result_values = [left_data[i] if i is not None else False 
                                       for i in result_indices["left"]]
                
                result_data[f"left_{col_name}"] = veloxx.PySeries(f"left_{col_name}", result_values)
        
        # Add right DataFrame columns
        for col_name in right_df.columns():
            right_series = right_df.get_column(col_name)
            try:
                right_data = right_series.to_vec_f64()
                result_values = [right_data[i] if i is not None else 0.0 
                               for i in result_indices["right"]]
            except:
                try:
                    right_data = right_series.to_vec_string()
                    result_values = [right_data[i] if i is not None else "" 
                                   for i in result_indices["right"]]
                except:
                    right_data = right_series.to_vec_bool()
                    result_values = [right_data[i] if i is not None else False 
                                   for i in result_indices["right"]]
            
            result_data[f"right_{col_name}"] = veloxx.PySeries(f"right_{col_name}", result_values)
        
        return veloxx.PyDataFrame(result_data)

    # Example: Join employee and department information
    dept_df = veloxx.PyDataFrame({
        "department": veloxx.PySeries("department", ["Engineering", "Sales", "HR"]),
        "budget": veloxx.PySeries("budget", [500000, 300000, 200000]),
        "manager": veloxx.PySeries("manager", ["Alice", "Bob", "Carol"])
    })

    # Perform join
    joined_df = join_dataframes(employee_df, dept_df, "department", "department", "inner")
    print("Joined employee and department data:")
    print(joined_df)

Error Handling and Debugging
----------------------------

Advanced Error Handling
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def safe_operation(operation_func, df, error_strategy="skip"):
        """Safely execute operations with error handling strategies"""
        try:
            return {
                "success": True,
                "result": operation_func(df),
                "error": None
            }
        except Exception as e:
            if error_strategy == "skip":
                return {
                    "success": False,
                    "result": df,  # Return original DataFrame
                    "error": str(e)
                }
            elif error_strategy == "raise":
                raise e
            elif error_strategy == "log":
                print(f"Operation failed: {str(e)}")
                return {
                    "success": False,
                    "result": df,
                    "error": str(e)
                }

    # Example: Safe filtering with error handling
    def risky_filter(df):
        # This might fail if column doesn't exist
        ages = df.get_column("nonexistent_column").to_vec_f64()
        return df.filter([i for i, age in enumerate(ages) if age > 30])

    result = safe_operation(risky_filter, employee_df, error_strategy="log")
    print(f"Operation successful: {result['success']}")
    if not result['success']:
        print(f"Error: {result['error']}")

Best Practices for Production Use
---------------------------------

1. **Memory Management**: Always consider memory usage for large datasets
2. **Error Handling**: Implement robust error handling for production systems
3. **Performance Monitoring**: Monitor operation performance and optimize bottlenecks
4. **Data Validation**: Validate input data before processing
5. **Documentation**: Document custom functions and pipelines thoroughly
6. **Testing**: Write tests for custom operations and pipelines
7. **Logging**: Implement proper logging for debugging and monitoring

These advanced features enable you to build robust, high-performance data processing applications with Veloxx while maintaining code quality and reliability.