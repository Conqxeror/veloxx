Quick Start Guide
=================

Get up and running with Veloxx in just a few minutes! This guide will walk you through the basics of using Veloxx for data processing.

Your First DataFrame
--------------------

Let's start by creating your first DataFrame:

.. code-block:: python

    import veloxx

    # Create a DataFrame with sample data
    df = veloxx.PyDataFrame({
        "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie", "David"]),
        "age": veloxx.PySeries("age", [25, 30, 22, 35]),
        "city": veloxx.PySeries("city", ["New York", "London", "New York", "Paris"]),
        "salary": veloxx.PySeries("salary", [50000.0, 75000.0, 45000.0, 80000.0])
    })

    print("Original DataFrame:")
    print(df)

This creates a DataFrame with 4 columns and 4 rows of sample employee data.

Basic Operations
----------------

Viewing Data
~~~~~~~~~~~~

.. code-block:: python

    # Display the DataFrame
    print(df)

    # Get DataFrame info
    print(f"Shape: {df.shape()}")  # (rows, columns)
    print(f"Columns: {df.columns()}")

Selecting Columns
~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Select specific columns
    names_and_ages = df.select_columns(["name", "age"])
    print("Names and Ages:")
    print(names_and_ages)

    # Get a single column as a Series
    age_series = df.get_column("age")
    print(f"Ages: {age_series.to_vec_f64()}")

Filtering Data
~~~~~~~~~~~~~~

.. code-block:: python

    # Filter rows where age > 25
    age_column = df.get_column("age")
    filtered_indices = [i for i, age in enumerate(age_column.to_vec_f64()) if age > 25]
    filtered_df = df.filter(filtered_indices)

    print("Employees older than 25:")
    print(filtered_df)

Series Operations
-----------------

Veloxx Series support various statistical operations:

.. code-block:: python

    age_series = df.get_column("age")
    salary_series = df.get_column("salary")

    # Basic statistics
    print(f"Average age: {age_series.mean()}")
    print(f"Total salary: {salary_series.sum()}")
    print(f"Max age: {age_series.max()}")
    print(f"Min salary: {salary_series.min()}")

    # Unique values
    unique_cities = df.get_column("city").unique()
    print(f"Unique cities: {unique_cities.to_vec_string()}")

Data Manipulation
-----------------

Renaming Columns
~~~~~~~~~~~~~~~~

.. code-block:: python

    # Rename a column
    renamed_df = df.rename_column("age", "years_old")
    print("DataFrame with renamed column:")
    print(renamed_df)

Adding New Columns
~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Create a new Series for experience
    experience = veloxx.PySeries("experience", [3, 8, 1, 12])
    
    # Add it to the DataFrame (this creates a new DataFrame)
    # Note: Direct column addition will be available in future versions
    extended_df = veloxx.PyDataFrame({
        "name": df.get_column("name"),
        "age": df.get_column("age"),
        "city": df.get_column("city"),
        "salary": df.get_column("salary"),
        "experience": experience
    })
    
    print("DataFrame with experience column:")
    print(extended_df)

Working with Different Data Types
----------------------------------

Veloxx supports multiple data types:

.. code-block:: python

    # String data
    names = veloxx.PySeries("names", ["Alice", "Bob", "Charlie"])
    
    # Numeric data (integers)
    ages = veloxx.PySeries("ages", [25, 30, 22])
    
    # Numeric data (floats)
    salaries = veloxx.PySeries("salaries", [50000.0, 75000.0, 45000.0])
    
    # Boolean data
    is_active = veloxx.PySeries("is_active", [True, False, True])

    mixed_df = veloxx.PyDataFrame({
        "names": names,
        "ages": ages,
        "salaries": salaries,
        "is_active": is_active
    })
    
    print("Mixed data types DataFrame:")
    print(mixed_df)

Common Patterns
---------------

Finding Top N Values
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Get employees with highest salaries
    salary_series = df.get_column("salary")
    salaries = salary_series.to_vec_f64()
    
    # Create list of (index, salary) pairs and sort
    indexed_salaries = [(i, sal) for i, sal in enumerate(salaries)]
    indexed_salaries.sort(key=lambda x: x[1], reverse=True)
    
    # Get top 2 highest paid employees
    top_indices = [idx for idx, _ in indexed_salaries[:2]]
    top_earners = df.filter(top_indices)
    
    print("Top 2 highest paid employees:")
    print(top_earners)

Grouping Data
~~~~~~~~~~~~~

.. code-block:: python

    # Group by city and calculate average salary
    cities = df.get_column("city").to_vec_string()
    salaries = df.get_column("salary").to_vec_f64()
    
    city_salaries = {}
    for city, salary in zip(cities, salaries):
        if city not in city_salaries:
            city_salaries[city] = []
        city_salaries[city].append(salary)
    
    # Calculate averages
    for city, sals in city_salaries.items():
        avg_salary = sum(sals) / len(sals)
        print(f"Average salary in {city}: ${avg_salary:,.2f}")

Next Steps
----------

Now that you've learned the basics, you can:

* Explore the :doc:`tutorial` for more detailed examples
* Check out the :doc:`examples` section for real-world use cases
* Read the :doc:`api` reference for complete function documentation
* Learn about :doc:`advanced` features for complex data processing

Performance Tips
----------------

* Use vectorized operations when possible
* Filter data early to reduce processing overhead
* Consider the data types you're using (integers vs floats)
* Reuse Series objects when creating multiple DataFrames with similar structure

Happy data processing with Veloxx! 🚀