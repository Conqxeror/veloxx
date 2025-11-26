import os

csv_path = 'src/io/csv.rs'
with open(csv_path, 'r', encoding='utf-8') as f:
    c = f.read()
c = c.replace('//!', '//')
with open(csv_path, 'w', encoding='utf-8') as f:
    f.write(c)

ef_path = 'src/performance/expression_fusion.rs'
with open(ef_path, 'r', encoding='utf-8') as f:
    c = f.read()

target_import = 'use rayon::prelude::*'
if 'use rayon::iter::IntoParallelRefMutIterator;' not in c:
    if target_import in c:
        c = c.replace(target_import, target_import + '\nuse rayon::iter::IntoParallelRefMutIterator;')
    else:
        c = 'use rayon::iter::IntoParallelRefMutIterator;\n' + c

with open(ef_path, 'w', encoding='utf-8') as f:
    f.write(c)

lazy_path = 'src/lazy.rs'
with open(lazy_path, 'r', encoding='utf-8') as f:
    c = f.read()

if 'fn evaluate_expr' in c and 'allow(unreachable_patterns)' not in c:
    c = c.replace('fn evaluate_expr', '#[allow(unreachable_patterns)]\n    fn evaluate_expr')

with open(lazy_path, 'w', encoding='utf-8') as f:
    f.write(c)

print("Fixes v3 applied.")