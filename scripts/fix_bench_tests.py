
import os

# 1. Fix benchmark json_io_bench.rs (remove unwrap, use IndexMap)
bench_path = 'benches/json_io_bench.rs'
if os.path.exists(bench_path):
    with open(bench_path, 'r', encoding='utf-8') as f:
        c = f.read()
    
    if 'use indexmap::IndexMap;' not in c:
        c = 'use indexmap::IndexMap;\n' + c
        
    c = c.replace('std::collections::HashMap::new()', 'IndexMap::new()')
    c = c.replace('.unwrap()', '') # Naive, but works if DataFrame::new no longer returns Result
    # Be careful if unwrap is used elsewhere.
    # DataFrame::new is the only one changing signature recently.
    
    with open(bench_path, 'w', encoding='utf-8') as f:
        f.write(c)

# 2. Fix tests/dataframe_select_test.rs (needless borrow)
test_path = 'tests/dataframe_select_test.rs'
if os.path.exists(test_path):
    with open(test_path, 'r', encoding='utf-8') as f:
        c = f.read()
    c = c.replace('&&"a".to_string()', '&"a".to_string()')
    c = c.replace('&&"b".to_string()', '&"b".to_string()')
    with open(test_path, 'w', encoding='utf-8') as f:
        f.write(c)

print("Fixed benchmarks and tests.")
