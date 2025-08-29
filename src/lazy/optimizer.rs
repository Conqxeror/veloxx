//! Query optimization module for lazy evaluation
//!
//! This module implements query optimization rules like predicate pushdown
//! and projection pushdown to improve performance of lazy DataFrames.

use crate::lazy::{Expr, LogicalPlan};

/// Query optimizer that applies various optimization rules
pub struct QueryOptimizer;

impl Default for QueryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryOptimizer {
    /// Create a new query optimizer
    pub fn new() -> Self {
        QueryOptimizer
    }

    /// Optimize a logical plan
    pub fn optimize(&self, plan: LogicalPlan) -> LogicalPlan {
        let plan = self.predicate_pushdown(plan);
        self.projection_pushdown(plan)
    }

    /// Push predicates down towards scan nodes
    #[allow(clippy::only_used_in_recursion)]
    fn predicate_pushdown(&self, plan: LogicalPlan) -> LogicalPlan {
        match plan {
            LogicalPlan::Filter { input, predicate } => {
                let optimized_input = self.predicate_pushdown(*input);

                match optimized_input {
                    LogicalPlan::DataFrameScan {
                        schema,
                        dataframe,
                        projection,
                        mut filters,
                    } => {
                        // Push the filter down to the scan node
                        filters.push(predicate);
                        LogicalPlan::DataFrameScan {
                            schema,
                            dataframe,
                            projection,
                            filters,
                        }
                    }
                    LogicalPlan::Projection {
                        input,
                        expr,
                        schema,
                    } => {
                        // Can't push predicate through projection, so keep the filter
                        LogicalPlan::Filter {
                            input: Box::new(LogicalPlan::Projection {
                                input,
                                expr,
                                schema,
                            }),
                            predicate,
                        }
                    }
                    _ => {
                        // For other node types, keep the filter where it is
                        LogicalPlan::Filter {
                            input: Box::new(optimized_input),
                            predicate,
                        }
                    }
                }
            }
            LogicalPlan::Projection {
                input,
                expr,
                schema,
            } => LogicalPlan::Projection {
                input: Box::new(self.predicate_pushdown(*input)),
                expr,
                schema,
            },
            LogicalPlan::DataFrameScan {
                schema,
                dataframe,
                projection,
                filters,
            } => LogicalPlan::DataFrameScan {
                schema,
                dataframe,
                projection,
                filters,
            },
            LogicalPlan::GroupBy {
                input,
                keys,
                aggregations,
                schema,
            } => LogicalPlan::GroupBy {
                input: Box::new(self.predicate_pushdown(*input)),
                keys,
                aggregations,
                schema,
            },
        }
    }

    /// Push projections down towards scan nodes
    #[allow(clippy::only_used_in_recursion)]
    fn projection_pushdown(&self, plan: LogicalPlan) -> LogicalPlan {
        match plan {
            LogicalPlan::Projection {
                input,
                expr,
                schema,
            } => {
                let optimized_input = self.projection_pushdown(*input);

                match optimized_input {
                    LogicalPlan::DataFrameScan {
                        schema: original_schema,
                        dataframe,
                        projection: _, // Ignore existing projection
                        filters,
                    } => {
                        // Extract column names from expressions
                        let mut column_names = Vec::new();
                        for e in &expr {
                            if let Expr::Column(name) = e {
                                column_names.push(name.clone());
                            }
                        }

                        // Push the projection down to the scan node
                        LogicalPlan::DataFrameScan {
                            schema: original_schema, // In a real implementation, we'd filter this
                            dataframe,
                            projection: Some(column_names),
                            filters,
                        }
                    }
                    _ => {
                        // For other node types, keep the projection where it is
                        LogicalPlan::Projection {
                            input: Box::new(optimized_input),
                            expr,
                            schema,
                        }
                    }
                }
            }
            LogicalPlan::Filter { input, predicate } => LogicalPlan::Filter {
                input: Box::new(self.projection_pushdown(*input)),
                predicate,
            },
            LogicalPlan::DataFrameScan {
                schema,
                dataframe,
                projection,
                filters,
            } => LogicalPlan::DataFrameScan {
                schema,
                dataframe,
                projection,
                filters,
            },
            LogicalPlan::GroupBy {
                input,
                keys,
                aggregations,
                schema,
            } => LogicalPlan::GroupBy {
                input: Box::new(self.projection_pushdown(*input)),
                keys,
                aggregations,
                schema,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataframe::DataFrame;
    use crate::lazy::{col, lit, BinaryOperator};
    use crate::series::Series;
    use crate::types::Value;
    use std::collections::HashMap;

    #[test]
    fn test_predicate_pushdown() {
        // Create a test DataFrame
        let mut columns = HashMap::new();
        columns.insert(
            "a".to_string(),
            Series::new_i32("a", vec![Some(1), Some(2), Some(3)]),
        );
        columns.insert(
            "b".to_string(),
            Series::new_f64("b", vec![Some(1.0), Some(2.0), Some(3.0)]),
        );
        let df = DataFrame::new(columns).unwrap();

        // Create a lazy DataFrame with filter
        let lazy_df = crate::lazy::LazyDataFrame::from_dataframe(df).filter(
            crate::lazy::binary_op(col("a"), BinaryOperator::Gt, lit(Value::I32(1))),
        );

        // Optimize the plan
        let optimizer = QueryOptimizer::new();
        let optimized_plan = optimizer.optimize(lazy_df.logical_plan);

        // Check that the filter was pushed down to the scan node
        match optimized_plan {
            LogicalPlan::DataFrameScan { filters, .. } => {
                assert_eq!(filters.len(), 1);
            }
            _ => panic!("Expected DataFrameScan after optimization"),
        }
    }

    #[test]
    fn test_projection_pushdown() {
        // Create a test DataFrame
        let mut columns = HashMap::new();
        columns.insert(
            "a".to_string(),
            Series::new_i32("a", vec![Some(1), Some(2), Some(3)]),
        );
        columns.insert(
            "b".to_string(),
            Series::new_f64("b", vec![Some(1.0), Some(2.0), Some(3.0)]),
        );
        let df = DataFrame::new(columns).unwrap();

        // Create a lazy DataFrame with projection
        let lazy_df = crate::lazy::LazyDataFrame::from_dataframe(df).select(vec![col("a")]);

        // Optimize the plan
        let optimizer = QueryOptimizer::new();
        let optimized_plan = optimizer.optimize(lazy_df.logical_plan);

        // Check that the projection was pushed down to the scan node
        match optimized_plan {
            LogicalPlan::DataFrameScan { projection, .. } => {
                assert!(projection.is_some());
                let proj_columns = projection.unwrap();
                assert_eq!(proj_columns.len(), 1);
                assert_eq!(proj_columns[0], "a");
            }
            _ => panic!("Expected DataFrameScan after optimization"),
        }
    }
}
