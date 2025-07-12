Python API Reference
====================

This section provides detailed documentation for all classes and functions in the Veloxx Python bindings.

.. automodule:: veloxx
   :members:
   :undoc-members:
   :show-inheritance:

Core Classes
------------

PyDataFrame
~~~~~~~~~~~

.. autoclass:: veloxx.PyDataFrame
   :members:
   :undoc-members:
   :show-inheritance:
   :no-index:

   The main data structure for working with tabular data in Veloxx. A DataFrame consists of named columns (Series) and provides methods for data manipulation, filtering, and analysis.

   **Constructor:**

   .. code-block:: python

       df = veloxx.PyDataFrame({
           "column1": veloxx.PySeries("column1", [1, 2, 3]),
           "column2": veloxx.PySeries("column2", ["a", "b", "c"])
       })

   **Key Methods:**

   * :meth:`~PyDataFrame.filter` - Filter rows by indices
   * :meth:`~PyDataFrame.select_columns` - Select specific columns
   * :meth:`~PyDataFrame.get_column` - Get a single column as Series
   * :meth:`~PyDataFrame.rename_column` - Rename a column
   * :meth:`~PyDataFrame.shape` - Get (rows, columns) dimensions
   * :meth:`~PyDataFrame.columns` - Get column names

PySeries
~~~~~~~~

.. autoclass:: veloxx.PySeries
   :members:
   :undoc-members:
   :show-inheritance:
   :no-index:

   Represents a column of data with a specific type. Series support various operations depending on their data type.

   **Constructor:**

   .. code-block:: python

       # Numeric series
       numbers = veloxx.PySeries("numbers", [1, 2, 3, 4, 5])
       
       # String series
       names = veloxx.PySeries("names", ["Alice", "Bob", "Charlie"])
       
       # Boolean series
       flags = veloxx.PySeries("flags", [True, False, True])

   **Key Methods:**

   * :meth:`~PySeries.to_vec_f64` - Convert to list of floats
   * :meth:`~PySeries.to_vec_string` - Convert to list of strings
   * :meth:`~PySeries.to_vec_bool` - Convert to list of booleans
   * :meth:`~PySeries.sum` - Sum of numeric values
   * :meth:`~PySeries.mean` - Average of numeric values
   * :meth:`~PySeries.min` - Minimum value
   * :meth:`~PySeries.max` - Maximum value
   * :meth:`~PySeries.unique` - Get unique values
   * :meth:`~PySeries.len` - Get length of series

Method Details
--------------

DataFrame Methods
~~~~~~~~~~~~~~~~~

filter(indices)
^^^^^^^^^^^^^^^

Filter the DataFrame to include only rows at the specified indices.

**Parameters:**
  * **indices** (*list of int*) - Row indices to keep

**Returns:**
  * **PyDataFrame** - New DataFrame with filtered rows

**Example:**

.. code-block:: python

    df = veloxx.PyDataFrame({
        "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie"]),
        "age": veloxx.PySeries("age", [25, 30, 22])
    })
    
    # Keep only first and last rows
    filtered = df.filter([0, 2])
    print(filtered)  # Shows Alice and Charlie

select_columns(column_names)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Create a new DataFrame with only the specified columns.

**Parameters:**
  * **column_names** (*list of str*) - Names of columns to keep

**Returns:**
  * **PyDataFrame** - New DataFrame with selected columns

**Example:**

.. code-block:: python

    selected = df.select_columns(["name"])
    print(selected)  # Shows only the name column

get_column(column_name)
^^^^^^^^^^^^^^^^^^^^^^^

Get a single column as a Series.

**Parameters:**
  * **column_name** (*str*) - Name of the column

**Returns:**
  * **PySeries** - The requested column

**Example:**

.. code-block:: python

    age_series = df.get_column("age")
    print(age_series.mean())  # Average age

rename_column(old_name, new_name)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Rename a column in the DataFrame.

**Parameters:**
  * **old_name** (*str*) - Current column name
  * **new_name** (*str*) - New column name

**Returns:**
  * **PyDataFrame** - New DataFrame with renamed column

**Example:**

.. code-block:: python

    renamed = df.rename_column("age", "years_old")
    print(renamed.columns())  # ['name', 'years_old']

shape()
^^^^^^^

Get the dimensions of the DataFrame.

**Returns:**
  * **tuple** - (number of rows, number of columns)

**Example:**

.. code-block:: python

    rows, cols = df.shape()
    print(f"DataFrame has {rows} rows and {cols} columns")

columns()
^^^^^^^^^

Get the column names of the DataFrame.

**Returns:**
  * **list of str** - Column names

**Example:**

.. code-block:: python

    col_names = df.columns()
    print(f"Columns: {col_names}")

Series Methods
~~~~~~~~~~~~~~

to_vec_f64()
^^^^^^^^^^^^

Convert the Series to a list of float values.

**Returns:**
  * **list of float** - Series values as floats

**Note:** Only works with numeric Series. String values will cause an error.

**Example:**

.. code-block:: python

    numbers = veloxx.PySeries("nums", [1, 2, 3])
    values = numbers.to_vec_f64()  # [1.0, 2.0, 3.0]

to_vec_string()
^^^^^^^^^^^^^^^

Convert the Series to a list of string values.

**Returns:**
  * **list of str** - Series values as strings

**Example:**

.. code-block:: python

    names = veloxx.PySeries("names", ["Alice", "Bob"])
    values = names.to_vec_string()  # ["Alice", "Bob"]

to_vec_bool()
^^^^^^^^^^^^^

Convert the Series to a list of boolean values.

**Returns:**
  * **list of bool** - Series values as booleans

**Note:** Only works with boolean Series.

**Example:**

.. code-block:: python

    flags = veloxx.PySeries("flags", [True, False, True])
    values = flags.to_vec_bool()  # [True, False, True]

sum()
^^^^^

Calculate the sum of numeric values in the Series.

**Returns:**
  * **float** - Sum of all values

**Note:** Only works with numeric Series.

**Example:**

.. code-block:: python

    numbers = veloxx.PySeries("nums", [1, 2, 3, 4])
    total = numbers.sum()  # 10.0

mean()
^^^^^^

Calculate the arithmetic mean of numeric values in the Series.

**Returns:**
  * **float** - Average of all values

**Note:** Only works with numeric Series.

**Example:**

.. code-block:: python

    numbers = veloxx.PySeries("nums", [1, 2, 3, 4])
    average = numbers.mean()  # 2.5

min()
^^^^^

Find the minimum value in the Series.

**Returns:**
  * **float** - Minimum value

**Note:** Only works with numeric Series.

**Example:**

.. code-block:: python

    numbers = veloxx.PySeries("nums", [3, 1, 4, 2])
    minimum = numbers.min()  # 1.0

max()
^^^^^

Find the maximum value in the Series.

**Returns:**
  * **float** - Maximum value

**Note:** Only works with numeric Series.

**Example:**

.. code-block:: python

    numbers = veloxx.PySeries("nums", [3, 1, 4, 2])
    maximum = numbers.max()  # 4.0

unique()
^^^^^^^^

Get unique values from the Series.

**Returns:**
  * **PySeries** - New Series containing only unique values

**Example:**

.. code-block:: python

    values = veloxx.PySeries("vals", [1, 2, 2, 3, 1])
    unique_vals = values.unique()
    print(unique_vals.to_vec_f64())  # [1.0, 2.0, 3.0]

len()
^^^^^

Get the number of elements in the Series.

**Returns:**
  * **int** - Number of elements

**Example:**

.. code-block:: python

    series = veloxx.PySeries("data", [1, 2, 3, 4, 5])
    length = series.len()  # 5

Type System
-----------

Veloxx automatically infers and handles data types:

Supported Types
~~~~~~~~~~~~~~~

* **String**: Text data (``str`` in Python)
* **Integer**: Whole numbers (converted to ``f64`` internally)
* **Float**: Decimal numbers (``f64`` internally)
* **Boolean**: True/False values (``bool`` in Python)

Type Conversion
~~~~~~~~~~~~~~~

Veloxx performs automatic type conversion when necessary:

.. code-block:: python

    # Mixed numeric types are converted to float
    mixed = veloxx.PySeries("mixed", [1, 2.5, 3])  # All become float
    
    # Strings remain strings
    text = veloxx.PySeries("text", ["hello", "world"])
    
    # Booleans remain booleans
    flags = veloxx.PySeries("flags", [True, False])

Error Handling
--------------

Common Errors
~~~~~~~~~~~~~

**TypeError**: Attempting to call numeric methods on non-numeric Series

.. code-block:: python

    names = veloxx.PySeries("names", ["Alice", "Bob"])
    # This will raise an error:
    # names.sum()  # TypeError: Cannot sum string Series

**IndexError**: Accessing non-existent columns or indices

.. code-block:: python

    df = veloxx.PyDataFrame({"name": veloxx.PySeries("name", ["Alice"])})
    # This will raise an error:
    # df.get_column("age")  # Column doesn't exist

**ValueError**: Creating DataFrame with mismatched Series lengths

.. code-block:: python

    # This will raise an error:
    # veloxx.PyDataFrame({
    #     "a": veloxx.PySeries("a", [1, 2]),
    #     "b": veloxx.PySeries("b", [1, 2, 3])  # Different length
    # })

Performance Notes
-----------------

* Veloxx is optimized for performance with large datasets
* Operations are implemented in Rust for maximum speed
* Memory usage is optimized through Rust's ownership system
* Consider filtering data early to reduce processing overhead
* Reuse Series objects when possible to minimize memory allocation

Version Information
-------------------

This documentation is for Veloxx Python bindings version 0.2.4.

For the latest updates and changes, see the `CHANGELOG <https://github.com/Conqxeror/veloxx/blob/main/CHANGELOG.md>`_.