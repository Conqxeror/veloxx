Examples and Use Cases
======================

This section provides practical examples of using Veloxx for real-world data processing tasks.

Data Analysis Examples
----------------------

Sales Data Analysis
~~~~~~~~~~~~~~~~~~~

Let's analyze a sales dataset to demonstrate common data analysis patterns:

.. code-block:: python

    import veloxx

    # Create sales data
    sales_df = veloxx.PyDataFrame({
        "product": veloxx.PySeries("product", [
            "Laptop", "Mouse", "Keyboard", "Monitor", "Laptop", 
            "Mouse", "Headphones", "Keyboard", "Monitor", "Laptop"
        ]),
        "region": veloxx.PySeries("region", [
            "North", "South", "North", "East", "West",
            "North", "South", "East", "West", "South"
        ]),
        "sales": veloxx.PySeries("sales", [
            1200.0, 25.0, 75.0, 300.0, 1150.0,
            30.0, 100.0, 80.0, 320.0, 1300.0
        ]),
        "quantity": veloxx.PySeries("quantity", [2, 5, 3, 1, 1, 6, 2, 4, 1, 2])
    })

    print("Sales Dataset:")
    print(sales_df)

**Total Sales by Product:**

.. code-block:: python

    # Group by product and calculate totals
    products = sales_df.get_column("product").to_vec_string()
    sales = sales_df.get_column("sales").to_vec_f64()
    quantities = sales_df.get_column("quantity").to_vec_f64()

    product_totals = {}
    for product, sale, qty in zip(products, sales, quantities):
        if product not in product_totals:
            product_totals[product] = {"sales": 0, "quantity": 0}
        product_totals[product]["sales"] += sale
        product_totals[product]["quantity"] += qty

    print("\nSales by Product:")
    for product, totals in sorted(product_totals.items(), 
                                  key=lambda x: x[1]["sales"], reverse=True):
        print(f"{product}: ${totals['sales']:,.2f} ({totals['quantity']} units)")

**Regional Performance:**

.. code-block:: python

    # Analyze regional performance
    regions = sales_df.get_column("region").to_vec_string()
    
    regional_sales = {}
    for region, sale in zip(regions, sales):
        if region not in regional_sales:
            regional_sales[region] = []
        regional_sales[region].append(sale)

    print("\nRegional Analysis:")
    for region, region_sales in regional_sales.items():
        total = sum(region_sales)
        avg = total / len(region_sales)
        print(f"{region}: Total=${total:,.2f}, Average=${avg:,.2f}, Orders={len(region_sales)}")

**High-Value Transactions:**

.. code-block:: python

    # Find high-value transactions (>$500)
    high_value_indices = [i for i, sale in enumerate(sales) if sale > 500]
    high_value_df = sales_df.filter(high_value_indices)

    print("\nHigh-Value Transactions (>$500):")
    print(high_value_df)

Employee Performance Analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Analyzing employee performance data:

.. code-block:: python

    # Employee performance dataset
    performance_df = veloxx.PyDataFrame({
        "employee_id": veloxx.PySeries("employee_id", [101, 102, 103, 104, 105, 106]),
        "name": veloxx.PySeries("name", [
            "Alice Johnson", "Bob Smith", "Carol Davis", 
            "David Wilson", "Eve Brown", "Frank Miller"
        ]),
        "department": veloxx.PySeries("department", [
            "Engineering", "Sales", "Engineering", "Sales", "HR", "Engineering"
        ]),
        "performance_score": veloxx.PySeries("performance_score", [92, 88, 95, 85, 90, 87]),
        "projects_completed": veloxx.PySeries("projects_completed", [12, 15, 10, 18, 8, 14]),
        "hours_worked": veloxx.PySeries("hours_worked", [2080, 2120, 2000, 2200, 1980, 2100])
    })

**Top Performers:**

.. code-block:: python

    # Find top performers (score >= 90)
    scores = performance_df.get_column("performance_score").to_vec_f64()
    top_performer_indices = [i for i, score in enumerate(scores) if score >= 90]
    top_performers = performance_df.filter(top_performer_indices)

    print("Top Performers (Score >= 90):")
    print(top_performers)

**Department Efficiency:**

.. code-block:: python

    # Calculate efficiency metrics by department
    departments = performance_df.get_column("department").to_vec_string()
    scores = performance_df.get_column("performance_score").to_vec_f64()
    projects = performance_df.get_column("projects_completed").to_vec_f64()
    hours = performance_df.get_column("hours_worked").to_vec_f64()

    dept_metrics = {}
    for dept, score, proj, hour in zip(departments, scores, projects, hours):
        if dept not in dept_metrics:
            dept_metrics[dept] = {
                "scores": [], "projects": [], "hours": [], "count": 0
            }
        dept_metrics[dept]["scores"].append(score)
        dept_metrics[dept]["projects"].append(proj)
        dept_metrics[dept]["hours"].append(hour)
        dept_metrics[dept]["count"] += 1

    print("\nDepartment Efficiency:")
    for dept, metrics in dept_metrics.items():
        avg_score = sum(metrics["scores"]) / len(metrics["scores"])
        total_projects = sum(metrics["projects"])
        avg_hours = sum(metrics["hours"]) / len(metrics["hours"])
        efficiency = total_projects / (avg_hours / 1000)  # Projects per 1000 hours
        
        print(f"{dept}:")
        print(f"  Employees: {metrics['count']}")
        print(f"  Avg Performance: {avg_score:.1f}")
        print(f"  Total Projects: {total_projects}")
        print(f"  Efficiency: {efficiency:.2f} projects/1000hrs")

Data Cleaning Examples
----------------------

Handling Missing or Invalid Data
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Dataset with some quality issues
    raw_data = veloxx.PyDataFrame({
        "customer_id": veloxx.PySeries("customer_id", [1, 2, 3, 4, 5, 6]),
        "age": veloxx.PySeries("age", [25, 0, 35, -5, 45, 30]),  # 0 and negative values
        "income": veloxx.PySeries("income", [50000, 75000, 0, 60000, 80000, 55000]),  # 0 income
        "city": veloxx.PySeries("city", ["New York", "", "Chicago", "Boston", "Seattle", ""])
    })

    print("Raw Data:")
    print(raw_data)

**Clean Invalid Ages:**

.. code-block:: python

    # Filter out invalid ages (<=0)
    ages = raw_data.get_column("age").to_vec_f64()
    valid_age_indices = [i for i, age in enumerate(ages) if age > 0]
    clean_age_df = raw_data.filter(valid_age_indices)

    print("\nData with Valid Ages:")
    print(clean_age_df)

**Filter Complete Records:**

.. code-block:: python

    # Get records with all fields populated
    cities = clean_age_df.get_column("city").to_vec_string()
    incomes = clean_age_df.get_column("income").to_vec_f64()
    
    complete_indices = [
        i for i, (city, income) in enumerate(zip(cities, incomes))
        if city.strip() != "" and income > 0
    ]
    
    complete_df = clean_age_df.filter(complete_indices)
    print("\nComplete Records Only:")
    print(complete_df)

Data Transformation Examples
----------------------------

Creating Derived Columns
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Financial data
    financial_df = veloxx.PyDataFrame({
        "company": veloxx.PySeries("company", ["TechCorp", "DataInc", "CloudCo", "AILabs"]),
        "revenue": veloxx.PySeries("revenue", [1000000, 750000, 1200000, 900000]),
        "expenses": veloxx.PySeries("expenses", [800000, 600000, 950000, 700000]),
        "employees": veloxx.PySeries("employees", [50, 30, 60, 40])
    })

**Calculate Profit and Margins:**

.. code-block:: python

    # Extract data for calculations
    companies = financial_df.get_column("company").to_vec_string()
    revenues = financial_df.get_column("revenue").to_vec_f64()
    expenses = financial_df.get_column("expenses").to_vec_f64()
    employees = financial_df.get_column("employees").to_vec_f64()

    # Calculate derived metrics
    profits = [rev - exp for rev, exp in zip(revenues, expenses)]
    margins = [profit / rev * 100 for profit, rev in zip(profits, revenues)]
    revenue_per_employee = [rev / emp for rev, emp in zip(revenues, employees)]

    # Create enhanced DataFrame
    enhanced_financial = veloxx.PyDataFrame({
        "company": financial_df.get_column("company"),
        "revenue": financial_df.get_column("revenue"),
        "expenses": financial_df.get_column("expenses"),
        "employees": financial_df.get_column("employees"),
        "profit": veloxx.PySeries("profit", profits),
        "margin_percent": veloxx.PySeries("margin_percent", margins),
        "revenue_per_employee": veloxx.PySeries("revenue_per_employee", revenue_per_employee)
    })

    print("Enhanced Financial Data:")
    print(enhanced_financial)

**Ranking and Categorization:**

.. code-block:: python

    # Rank companies by profitability
    company_profits = list(zip(companies, profits))
    company_profits.sort(key=lambda x: x[1], reverse=True)

    print("\nCompanies Ranked by Profit:")
    for i, (company, profit) in enumerate(company_profits, 1):
        print(f"{i}. {company}: ${profit:,.2f}")

    # Categorize by size
    size_categories = []
    for emp_count in employees:
        if emp_count < 40:
            size_categories.append("Small")
        elif emp_count < 60:
            size_categories.append("Medium")
        else:
            size_categories.append("Large")

    # Add size category
    categorized_df = veloxx.PyDataFrame({
        "company": financial_df.get_column("company"),
        "employees": financial_df.get_column("employees"),
        "size_category": veloxx.PySeries("size_category", size_categories),
        "profit": veloxx.PySeries("profit", profits)
    })

    print("\nCompanies by Size Category:")
    print(categorized_df)

Time Series Analysis Example
----------------------------

Monthly Sales Tracking
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Monthly sales data
    monthly_sales = veloxx.PyDataFrame({
        "month": veloxx.PySeries("month", [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
        ]),
        "sales_2023": veloxx.PySeries("sales_2023", [
            85000, 92000, 88000, 95000, 102000, 98000,
            105000, 110000, 95000, 100000, 115000, 125000
        ]),
        "sales_2024": veloxx.PySeries("sales_2024", [
            90000, 95000, 93000, 100000, 108000, 105000,
            112000, 118000, 102000, 108000, 122000, 135000
        ])
    })

**Calculate Growth Rates:**

.. code-block:: python

    sales_2023 = monthly_sales.get_column("sales_2023").to_vec_f64()
    sales_2024 = monthly_sales.get_column("sales_2024").to_vec_f64()
    months = monthly_sales.get_column("month").to_vec_string()

    # Calculate month-over-month growth for 2024
    growth_rates = [(s24 - s23) / s23 * 100 for s23, s24 in zip(sales_2023, sales_2024)]

    # Find best and worst performing months
    month_growth = list(zip(months, growth_rates))
    month_growth.sort(key=lambda x: x[1], reverse=True)

    print("Year-over-Year Growth by Month:")
    for month, growth in month_growth:
        print(f"{month}: {growth:+.1f}%")

    # Calculate quarterly totals
    quarters = {
        "Q1": sum(sales_2024[0:3]),
        "Q2": sum(sales_2024[3:6]),
        "Q3": sum(sales_2024[6:9]),
        "Q4": sum(sales_2024[9:12])
    }

    print(f"\n2024 Quarterly Sales:")
    for quarter, total in quarters.items():
        print(f"{quarter}: ${total:,.2f}")

Performance Optimization Examples
---------------------------------

Efficient Data Processing
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Large dataset simulation
    import random

    # Generate larger dataset for performance testing
    def generate_large_dataset(size=1000):
        products = ["Product_A", "Product_B", "Product_C", "Product_D", "Product_E"]
        regions = ["North", "South", "East", "West", "Central"]
        
        data = {
            "id": list(range(1, size + 1)),
            "product": [random.choice(products) for _ in range(size)],
            "region": [random.choice(regions) for _ in range(size)],
            "sales": [random.uniform(10, 1000) for _ in range(size)],
            "quantity": [random.randint(1, 10) for _ in range(size)]
        }
        
        return veloxx.PyDataFrame({
            "id": veloxx.PySeries("id", data["id"]),
            "product": veloxx.PySeries("product", data["product"]),
            "region": veloxx.PySeries("region", data["region"]),
            "sales": veloxx.PySeries("sales", data["sales"]),
            "quantity": veloxx.PySeries("quantity", data["quantity"])
        })

    # Create large dataset
    large_df = generate_large_dataset(1000)
    print(f"Generated dataset with {large_df.shape()[0]} rows")

**Efficient Filtering:**

.. code-block:: python

    # Filter for high-value transactions efficiently
    sales = large_df.get_column("sales").to_vec_f64()
    
    # Single pass through data
    high_value_indices = []
    total_high_value = 0
    count_high_value = 0
    
    for i, sale in enumerate(sales):
        if sale > 500:
            high_value_indices.append(i)
            total_high_value += sale
            count_high_value += 1

    print(f"High-value transactions: {count_high_value}")
    print(f"Total high-value sales: ${total_high_value:,.2f}")

    # Get the filtered dataset
    high_value_df = large_df.filter(high_value_indices)
    print(f"Filtered dataset size: {high_value_df.shape()[0]} rows")

Integration Examples
--------------------

Working with Python Ecosystem
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    # Convert Veloxx data to work with other libraries
    def veloxx_to_dict(df):
        """Convert Veloxx DataFrame to Python dictionary"""
        result = {}
        for col_name in df.columns():
            series = df.get_column(col_name)
            try:
                # Try numeric conversion first
                result[col_name] = series.to_vec_f64()
            except:
                try:
                    # Try string conversion
                    result[col_name] = series.to_vec_string()
                except:
                    # Try boolean conversion
                    result[col_name] = series.to_vec_bool()
        return result

    # Example usage
    sample_df = veloxx.PyDataFrame({
        "name": veloxx.PySeries("name", ["Alice", "Bob", "Charlie"]),
        "age": veloxx.PySeries("age", [25, 30, 35]),
        "active": veloxx.PySeries("active", [True, False, True])
    })

    # Convert to dictionary for use with other libraries
    data_dict = veloxx_to_dict(sample_df)
    print("Converted to dictionary:")
    for key, values in data_dict.items():
        print(f"{key}: {values}")

**Export to CSV-like format:**

.. code-block:: python

    def export_to_csv_string(df):
        """Export DataFrame to CSV-formatted string"""
        lines = []
        
        # Header
        lines.append(",".join(df.columns()))
        
        # Data rows
        num_rows = df.shape()[0]
        for i in range(num_rows):
            row_values = []
            for col_name in df.columns():
                series = df.get_column(col_name)
                try:
                    value = series.to_vec_f64()[i]
                    row_values.append(str(value))
                except:
                    try:
                        value = series.to_vec_string()[i]
                        row_values.append(f'"{value}"')
                    except:
                        value = series.to_vec_bool()[i]
                        row_values.append(str(value).lower())
            lines.append(",".join(row_values))
        
        return "\n".join(lines)

    # Export sample data
    csv_output = export_to_csv_string(sample_df)
    print("\nCSV Export:")
    print(csv_output)

Best Practices Summary
----------------------

1. **Filter Early**: Apply filters as early as possible to reduce data size
2. **Reuse Objects**: Reuse Series objects when creating multiple DataFrames
3. **Type Awareness**: Be aware of data types for optimal performance
4. **Memory Management**: Process large datasets in chunks if needed
5. **Error Handling**: Always handle potential type mismatches gracefully

These examples demonstrate the versatility and power of Veloxx for various data processing tasks. The library's Rust-based performance makes it ideal for handling large datasets efficiently while maintaining a familiar Python interface.