import os

def fix_bench_file(fp):
    if not os.path.exists(fp): return
    with open(fp, 'r', encoding='utf-8') as f:
        c = f.read()
    
    # Ensure indexmap import
    if 'use indexmap::IndexMap;' not in c:
        # find last use
        lines = c.splitlines()
        idx = 0
        for i, line in enumerate(lines):
            if line.startswith('use '):
                idx = i
        lines.insert(idx+1, 'use indexmap::IndexMap;')
        c = '\n'.join(lines)

    # Replace HashMap with IndexMap
    c = c.replace('std::collections::HashMap::new()', 'IndexMap::new()')
    c = c.replace('HashMap::new()', 'IndexMap::new()')
    
    # Remove .unwrap() from DataFrame::new().unwrap()
    # Be careful not to remove other unwraps.
    # Regex: DataFrame::new\((.*?)\)\.unwrap\(\) -> DataFrame::new(\1)
    
    # Simple string replacement for common case
    c = c.replace('DataFrame::new(columns).unwrap()', 'DataFrame::new(columns)')
    c = c.replace('DataFrame::new(columns1).unwrap()', 'DataFrame::new(columns1)')
    c = c.replace('DataFrame::new(columns2).unwrap()', 'DataFrame::new(columns2)')
    
    with open(fp, 'w', encoding='utf-8') as f:
        f.write(c)

bench_files = [
    'benches/groupby_sum_bench.rs',
    'benches/window_ranking_bench.rs',
    'benches/core_operations_bench.rs',
    'benches/comprehensive_benchmarks.rs',
    'benches/comprehensive_final_benchmark.rs',
    'benches/arrow_math_benchmarks.rs'
]

for b in bench_files:
    fix_bench_file(b)

# Fix arrow_math_benchmarks.rs type annotations
# It needs `|b, (s1, s2): &(_, _)|` instead of `|b, (s1, s2)|`
fp_arrow = 'benches/arrow_math_benchmarks.rs'
if os.path.exists(fp_arrow):
    with open(fp_arrow, 'r', encoding='utf-8') as f:
        c = f.read()
    c = c.replace('|b, (s1, s2)|', '|b, (s1, s2): &(_, _)|')
    # Fix import error `use veloxx::arrow::{ArrowOps, ArrowSeries};` -> `use veloxx::series::Series;` ?
    # arrow module is guarded or structure changed.
    # Actually `ArrowOps` trait might be gone or renamed?
    # Series now implements `arrow_add`, `arrow_sub` directly.
    # I'll remove the broken import and ensure `Series` is imported.
    if 'use veloxx::arrow::{ArrowOps, ArrowSeries};' in c:
        c = c.replace('use veloxx::arrow::{ArrowOps, ArrowSeries};', 'use veloxx::series::Series;')
    
    with open(fp_arrow, 'w', encoding='utf-8') as f:
        f.write(c)

print("Fixed remaining benchmarks.")
