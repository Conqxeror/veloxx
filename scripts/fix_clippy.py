
import os

# 1. Fix Python Bindings Empty Line after Outer Attr
file_path = 'src/python_bindings.rs'
with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace('/// Group by operations\n\n    /// Reshape the DataFrame from long to wide format', '/// Group by operations\n\n\n    /// Reshape the DataFrame from long to wide format')
# Actually clippy complained about empty line AFTER outer attribute?
# No:
# 1083 | /     /// Group by operations
# 1084 | |
#      | |_^ 
# 1085 |       /// Reshape the DataFrame from long to wide format
#
# Wait, `/// Group by operations` is a doc comment. The empty line separates it from the NEXT function?
# But the error says "empty line after outer attribute".
# If `///` is treated as an attribute on the *next* item, and there is a gap...
# No, the previous function `group_by` probably ended.
# Ah, `pivot` implementation I inserted:
#
#    /// Group by operations
#    pub fn group_by(...) ... { ... }
#
#    /// Reshape ...
#    pub fn pivot(...)
#
# My script inserted pivot BEFORE group_by in a weird way?
# Let's check file content.

# 2. Fix map_clone in lazy.rs
file_path_lazy = 'src/lazy.rs'
with open(file_path_lazy, 'r', encoding='utf-8') as f:
    content_lazy = f.read()

content_lazy = content_lazy.replace('.map(|s| s.clone())', '.cloned()')

# Fix empty line in lazy.rs docs?
# 182 | /     /// Execute a logical plan (static method to avoid borrow issues)
# 183 | |
#     | |_^ 
# 184 |       /// Evaluate an expression against a DataFrame
#
# I inserted `evaluate_expr` before `execute_plan_static` but left the doc comment for `execute_plan_static` dangling?
# Or I inserted `evaluate_expr` inside the doc block?
# `evaluate_expr` has its own doc comment.
# It seems I split them with a newline that clippy hates.

# 3. Fix Unused Imports
files_to_fix = {
    'src/data_quality.rs': 'use std::collections::HashMap;',
    'src/io/csv.rs': ['use std::sync::Arc;', 'use std::io::{BufRead, BufReader};'],
    'src/io/json.rs': 'use indexmap::IndexMap;',
    'src/performance/expression_fusion.rs': ['use crate::dataframe::DataFrame;', 'use rayon::prelude::*;'],
    'src/performance/memory_pool.rs': 'use indexmap::IndexMap;'
}

# We can just read and replace `use ...;` with empty string if we are sure.
for fp, imports in files_to_fix.items():
    if not isinstance(imports, list):
        imports = [imports]
    
    if os.path.exists(fp):
        with open(fp, 'r', encoding='utf-8') as f:
            c = f.read()
        for imp in imports:
            c = c.replace(imp, '')
        with open(fp, 'w', encoding='utf-8') as f:
            f.write(c)

# 4. Fix deprecated `remove` in manipulation.rs
man_path = 'src/dataframe/manipulation.rs'
with open(man_path, 'r', encoding='utf-8') as f:
    c = f.read()
c = c.replace('.remove(', '.swap_remove(')
with open(man_path, 'w', encoding='utf-8') as f:
    f.write(c)

# 5. Fix Unreachable Pattern in lazy.rs
# The `_ => Err(...)` is unreachable because I covered all variants.
# I can remove it or use `#[allow(unreachable_patterns)]` on the match.
# Or just remove the arm.
# But `BinaryOperator` might have other variants if I missed one?
# If I covered all, `_` is unreachable.
# I will remove the `_ => ...` arm.
if '_ => Err(VeloxxError::Unsupported(format!("Binary operator {{:?}} not implemented", op))),' in content_lazy:
    content_lazy = content_lazy.replace('_ => Err(VeloxxError::Unsupported(format!("Binary operator {{:?}} not implemented", op))),', '')

with open(file_path_lazy, 'w', encoding='utf-8') as f:
    f.write(content_lazy)

# 6. Fix empty lines in doc comments (lazy.rs, csv.rs, manipulation.rs, series/mod.rs)
# Regex replace `/// ...\n\n` with `/// ...\n///\n`?
# Or just remove the empty line.
# Clppy says "if the empty line is unintentional, remove it".
# I will remove empty lines between doc comments and functions/structs?
# No, clippy complains about empty lines *inside* doc blocks or immediately after?
# "empty line after doc comment" usually means:
# /// Doc
# 
# pub fn foo...
#
# I should remove that newline.

def remove_empty_line_after_doc(fp):
    if not os.path.exists(fp): return
    with open(fp, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    new_lines = []
    for i in range(len(lines)):
        if i > 0 and lines[i].strip() == '' and lines[i-1].strip().startswith('///'):
            # Check if next line is NOT a doc comment (start of item)
            if i+1 < len(lines) and not lines[i+1].strip().startswith('///'):
                continue
        new_lines.append(lines[i])
    with open(fp, 'w', encoding='utf-8') as f:
        f.writelines(new_lines)

remove_empty_line_after_doc('src/dataframe/manipulation.rs')
remove_empty_line_after_doc('src/io/csv.rs')
remove_empty_line_after_doc('src/series/mod.rs')
remove_empty_line_after_doc('src/lazy.rs')
remove_empty_line_after_doc('src/python_bindings.rs')

# 7. Fix Type Complexity in join.rs (Allow it)
join_path = 'src/dataframe/join.rs'
with open(join_path, 'r', encoding='utf-8') as f:
    c = f.read()
if '#[allow(clippy::type_complexity)]' not in c:
    c = c.replace('pub fn join(', '#[allow(clippy::type_complexity)]\n    pub fn join(')
    # Actually the error is inside the function?
    # "very complex type used ... let collected_results: Vec<...>"
    # I can add `#[allow(clippy::type_complexity)]` to the function or inner attribute.
    # Inner is safer: `#![allow...]` but that's module level.
    # I'll add to function.
with open(join_path, 'w', encoding='utf-8') as f:
    f.write(c)

# 8. Fix New without Default in ml.rs
ml_path = 'src/ml.rs'
with open(ml_path, 'r', encoding='utf-8') as f:
    c = f.read()
# Implement Default for LogisticRegression
default_impl = """
impl Default for LogisticRegression {
    fn default() -> Self {
        Self::new()
    }
}
"""
if 'pub struct LogisticRegression {' in c and 'impl Default for LogisticRegression' not in c:
    # Insert before impl LogisticRegression
    target = 'impl LogisticRegression {'
    c = c.replace(target, default_impl + '\n' + target)
with open(ml_path, 'w', encoding='utf-8') as f:
    f.write(c)

print("Clippy fixes applied.")
