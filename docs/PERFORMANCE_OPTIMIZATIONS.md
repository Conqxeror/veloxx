# Performance Optimizations

This document outlines the performance optimizations that have been applied to the Veloxx library.

## Series Module

- **`filter` operation:** The `filter` operation has been parallelized using the `rayon` library. This can provide a significant performance improvement for large Series.
- **`mean` calculation:** The `mean` calculation has been parallelized using the `rayon` library. This can provide a significant performance improvement for large Series.

## DataFrame Module

- **`filter_by_indices` operation:** The `filter_by_indices` operation has been parallelized using the `rayon` library. This can provide a significant performance improvement for large DataFrames.
- **`drop_nulls` operation:** The `drop_nulls` operation has been parallelized using the `rayon` library. This can provide a significant performance improvement for large DataFrames.
