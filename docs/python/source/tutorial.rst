Complete Tutorial
=================

This comprehensive tutorial will guide you through all aspects of using Veloxx Python bindings, from basic concepts to advanced data processing techniques.

Understanding Veloxx Architecture
----------------------------------

Veloxx is built with a Rust core for maximum performance and memory safety. The Python bindings provide a familiar interface while leveraging Rust's speed and safety guarantees.

Core Components
~~~~~~~~~~~~~~~

* **PyDataFrame**: The main data structure for tabular data
* **PySeries**: Column data structure with type-specific operations
* **Type System**: Automatic type inference with support for strings, integers, floats, and booleans

Creating DataFrames and Series
-------------------------------

From Python Data Structures
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    import veloxx

    # From lists
    names = ["Alice", "Bob", "Charlie", "Diana"]
    ages = [25, 30, 22, 28]
    
    df = veloxx.PyDataFrame({
        "name": veloxx.PySeries("name", names),
        "age": veloxx.PySeries("age", ages)
    })

From Mixed Data Types
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Creating a more complex DataFrame
    employee_data = veloxx.PyDataFrame({
        "id": veloxx.PySeries("id", [1, 2, 3, 4, 5]),
        "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie", "Diana", "Eve"]),
        "department": veloxx.PySeries("department", ["Engineering", "Sales", "Engineering", "HR", "Sales"]),
        "salary": veloxx.PySeries("salary", [75000.0, 65000.0, 80000.0, 60000.0, 70000.0]),
        "is_manager": veloxx.PySeries("is_manager", [False, True, False, True, False]),
        "years_experience": veloxx.PySeries("years_experience", [3, 8, 5, 10, 2])
    })
    
    print("Employee DataFrame:")
    print(employee_data)

Data Exploration and Inspection
--------------------------------

Basic Information
~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Get basic information about the DataFrame
    print(f"Shape: {employee_data.shape()}")
    print(f"Columns: {employee_data.columns()}")
    
    # Examine individual columns
    for col_name in employee_data.columns():
        series = employee_data.get_column(col_name)
        print(f"\nColumn '{col_name}':")
        print(f"  Length: {series.len()}")
        
        # Type-specific operations
        if col_name in ["salary", "years_experience"]:
            print(f"  Sum: {series.sum()}")
            print(f"  Mean: {series.mean()}")
            print(f"  Min: {series.min()}")
            print(f"  Max: {series.max()}")

Statistical Analysis
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Salary analysis
    salary_series = employee_data.get_column("salary")
    print("Salary Statistics:")
    print(f"  Total payroll: ${salary_series.sum():,.2f}")
    print(f"  Average salary: ${salary_series.mean():,.2f}")
    print(f"  Salary range: ${salary_series.min():,.2f} - ${salary_series.max():,.2f}")
    
    # Experience analysis
    exp_series = employee_data.get_column("years_experience")
    print(f"\nExperience Statistics:")
    print(f"  Average experience: {exp_series.mean():.1f} years")
    print(f"  Total team experience: {exp_series.sum()} years")

Data Filtering and Selection
----------------------------

Column Selection
~~~~~~~~~~~~~~~~

.. code-block:: python

    # Select specific columns
    basic_info = employee_data.select_columns(["name", "department", "salary"])
    print("Basic employee information:")
    print(basic_info)
    
    # Select columns by pattern (manual implementation)
    numeric_columns = ["salary", "years_experience"]
    numeric_data = employee_data.select_columns(numeric_columns)
    print("\nNumeric data only:")
    print(numeric_data)

Row Filtering
~~~~~~~~~~~~~

.. code-block:: python

    # Filter high earners (salary > 70000)
    salaries = employee_data.get_column("salary").to_vec_f64()
    high_earner_indices = [i for i, salary in enumerate(salaries) if salary > 70000]
    high_earners = employee_data.filter(high_earner_indices)
    
    print("High earners (>$70,000):")
    print(high_earners)
    
    # Filter by department
    departments = employee_data.get_column("department").to_vec_string()
    engineering_indices = [i for i, dept in enumerate(departments) if dept == "Engineering"]
    engineering_team = employee_data.filter(engineering_indices)
    
    print("\nEngineering team:")
    print(engineering_team)
    
    # Complex filtering: Senior engineers (Engineering + experience > 4)
    experience = employee_data.get_column("years_experience").to_vec_f64()
    senior_eng_indices = [
        i for i, (dept, exp) in enumerate(zip(departments, experience))
        if dept == "Engineering" and exp > 4
    ]
    senior_engineers = employee_data.filter(senior_eng_indices)
    
    print("\nSenior engineers:")
    print(senior_engineers)

Data Transformation
-------------------

Column Operations
~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Rename columns
    renamed_df = employee_data.rename_column("years_experience", "experience_years")
    print("DataFrame with renamed column:")
    print(renamed_df.columns())
    
    # Create derived data
    salaries = employee_data.get_column("salary").to_vec_f64()
    experience = employee_data.get_column("years_experience").to_vec_f64()
    
    # Calculate salary per year of experience
    salary_per_exp = [sal / max(exp, 1) for sal, exp in zip(salaries, experience)]
    
    # Create new DataFrame with calculated column
    enhanced_df = veloxx.PyDataFrame({
        "name": employee_data.get_column("name"),
        "department": employee_data.get_column("department"),
        "salary": employee_data.get_column("salary"),
        "experience": employee_data.get_column("years_experience"),
        "salary_per_exp_year": veloxx.PySeries("salary_per_exp_year", salary_per_exp)
    })
    
    print("\nEnhanced DataFrame with calculated column:")
    print(enhanced_df)

Data Aggregation and Grouping
------------------------------

Manual Grouping
~~~~~~~~~~~~~~~

.. code-block:: python

    # Group by department and calculate statistics
    departments = employee_data.get_column("department").to_vec_string()
    salaries = employee_data.get_column("salary").to_vec_f64()
    experience = employee_data.get_column("years_experience").to_vec_f64()
    
    # Create department groups
    dept_groups = {}
    for i, dept in enumerate(departments):
        if dept not in dept_groups:
            dept_groups[dept] = {"salaries": [], "experience": [], "count": 0}
        dept_groups[dept]["salaries"].append(salaries[i])
        dept_groups[dept]["experience"].append(experience[i])
        dept_groups[dept]["count"] += 1
    
    # Calculate department statistics
    print("Department Statistics:")
    for dept, data in dept_groups.items():
        avg_salary = sum(data["salaries"]) / len(data["salaries"])
        avg_experience = sum(data["experience"]) / len(data["experience"])
        total_salary = sum(data["salaries"])
        
        print(f"\n{dept}:")
        print(f"  Employees: {data['count']}")
        print(f"  Average salary: ${avg_salary:,.2f}")
        print(f"  Average experience: {avg_experience:.1f} years")
        print(f"  Total department salary: ${total_salary:,.2f}")

Advanced Filtering Techniques
------------------------------

Multiple Conditions
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Find employees who are either managers OR have high salary
    salaries = employee_data.get_column("salary").to_vec_f64()
    is_manager = employee_data.get_column("is_manager").to_vec_bool()
    
    high_value_indices = [
        i for i, (salary, manager) in enumerate(zip(salaries, is_manager))
        if manager or salary > 75000
    ]
    
    high_value_employees = employee_data.filter(high_value_indices)
    print("High-value employees (managers or high salary):")
    print(high_value_employees)

Custom Filter Functions
~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    def create_filter(df, filter_func):
        """Create a filter based on a custom function"""
        indices = []
        for i in range(len(df.get_column(df.columns()[0]).to_vec_string())):
            # Extract row data
            row_data = {}
            for col in df.columns():
                series = df.get_column(col)
                if col == "name" or col == "department":
                    row_data[col] = series.to_vec_string()[i]
                elif col == "is_manager":
                    row_data[col] = series.to_vec_bool()[i]
                else:
                    row_data[col] = series.to_vec_f64()[i]
            
            if filter_func(row_data):
                indices.append(i)
        
        return df.filter(indices)
    
    # Use custom filter
    def is_senior_employee(row):
        return row["years_experience"] >= 5 and row["salary"] >= 70000
    
    senior_employees = create_filter(employee_data, is_senior_employee)
    print("Senior employees (5+ years experience and $70k+ salary):")
    print(senior_employees)

Working with Series
-------------------

Series Creation and Manipulation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Create different types of Series
    text_series = veloxx.PySeries("cities", ["New York", "London", "Tokyo", "Paris"])
    number_series = veloxx.PySeries("populations", [8.4, 9.0, 13.9, 2.1])  # in millions
    
    print("Text Series:")
    print(f"Values: {text_series.to_vec_string()}")
    print(f"Unique values: {text_series.unique().to_vec_string()}")
    
    print("\nNumber Series:")
    print(f"Values: {number_series.to_vec_f64()}")
    print(f"Sum: {number_series.sum()}")
    print(f"Average: {number_series.mean()}")

Series Operations
~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Mathematical operations on numeric series
    sales_q1 = veloxx.PySeries("q1_sales", [100000, 150000, 120000, 180000])
    sales_q2 = veloxx.PySeries("q2_sales", [110000, 160000, 115000, 190000])
    
    q1_values = sales_q1.to_vec_f64()
    q2_values = sales_q2.to_vec_f64()
    
    # Calculate growth
    growth_rates = [(q2 - q1) / q1 * 100 for q1, q2 in zip(q1_values, q2_values)]
    growth_series = veloxx.PySeries("growth_rate", growth_rates)
    
    print("Sales Analysis:")
    print(f"Q1 total: ${sales_q1.sum():,.2f}")
    print(f"Q2 total: ${sales_q2.sum():,.2f}")
    print(f"Average growth rate: {growth_series.mean():.2f}%")

Best Practices and Performance Tips
-----------------------------------

Efficient Data Processing
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # DO: Filter early to reduce data size
    # Filter first, then perform operations
    large_dataset = employee_data  # Imagine this is much larger
    
    # Filter to relevant subset first
    departments = large_dataset.get_column("department").to_vec_string()
    eng_indices = [i for i, dept in enumerate(departments) if dept == "Engineering"]
    eng_subset = large_dataset.filter(eng_indices)
    
    # Then perform expensive operations on smaller dataset
    eng_salaries = eng_subset.get_column("salary")
    avg_eng_salary = eng_salaries.mean()
    
    print(f"Average Engineering salary: ${avg_eng_salary:,.2f}")

Memory Management
~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Reuse Series objects when possible
    name_series = veloxx.PySeries("name", ["Alice", "Bob", "Charlie"])
    
    # Use the same series in multiple DataFrames
    df1 = veloxx.PyDataFrame({
        "name": name_series,
        "score1": veloxx.PySeries("score1", [85, 92, 78])
    })
    
    df2 = veloxx.PyDataFrame({
        "name": name_series,  # Reuse the same series
        "score2": veloxx.PySeries("score2", [88, 94, 82])
    })

Error Handling
--------------

Common Issues and Solutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    try:
        # Attempt to create DataFrame with mismatched lengths
        names = veloxx.PySeries("names", ["Alice", "Bob"])
        ages = veloxx.PySeries("ages", [25, 30, 22])  # Different length!
        
        df = veloxx.PyDataFrame({"names": names, "ages": ages})
    except Exception as e:
        print(f"Error creating DataFrame: {e}")
        
        # Fix: Ensure all series have the same length
        names = veloxx.PySeries("names", ["Alice", "Bob", "Charlie"])
        ages = veloxx.PySeries("ages", [25, 30, 22])
        df = veloxx.PyDataFrame({"names": names, "ages": ages})
        print("DataFrame created successfully!")

Type Safety
~~~~~~~~~~~

.. code-block:: python

    # Veloxx automatically handles type inference
    mixed_numbers = [1, 2.5, 3, 4.7]  # Mix of int and float
    series = veloxx.PySeries("mixed", mixed_numbers)
    
    # All values are converted to float for consistency
    print(f"Values: {series.to_vec_f64()}")
    print(f"Type: float64 (automatic conversion)")

Next Steps
----------

You've now learned the fundamentals of Veloxx! To continue your journey:

1. Explore the :doc:`examples` section for real-world use cases
2. Check the :doc:`api` reference for complete function documentation
3. Learn about :doc:`advanced` features for complex data processing
4. Consider contributing to the project on `GitHub <https://github.com/Conqxeror/veloxx>`_

Remember: Veloxx is designed for performance and safety. While the API might feel familiar if you're coming from pandas, the underlying Rust implementation provides significant performance benefits for large datasets and compute-intensive operations.