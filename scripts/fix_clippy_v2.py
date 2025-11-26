
import os

# 1. Restore BufRead import in io/csv.rs (I removed too much!)
csv_path = 'src/io/csv.rs'
with open(csv_path, 'r', encoding='utf-8') as f:
    c = f.read()
# I replaced `use std::io::{BufRead, BufReader};` with empty string.
# But `read_from_reader` uses `BufRead` trait bound. `BufReader` is used in `read_file`.
# I removed `BufReader` usage in `read_file` earlier when switching to memmap, but `read_from_reader` needs `BufRead`.
# And `read_file` now uses `File::open`, `Mmap`, then `parse_bytes`.
# `read_from_reader` uses `reader.lines()`.
# So I need `use std::io::BufRead;`.

if 'use std::io::BufRead;' not in c:
    c = 'use std::io::BufRead;\n' + c

with open(csv_path, 'w', encoding='utf-8') as f:
    f.write(c)

# 2. Fix empty line doc comments properly
# My previous regex logic might have been flawed or not strict enough.
# The error shows:
# 617 | /     /// ```
# 618 | |
#     | |_^
# 619 |       /// Filter DataFrame ...
#
# It means I have:
# /// ```
#
# /// Filter ...
#
# I should remove that blank line.

def strict_remove_empty_doc_line(fp):
    if not os.path.exists(fp): return
    with open(fp, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    new_lines = []
    for i in range(len(lines)):
        # If current line is empty (whitespace), and PREV line was doc comment, and NEXT line is doc comment
        # Then remove it.
        if i > 0 and i < len(lines) - 1:
            curr = lines[i].strip()
            prev = lines[i-1].strip()
            next_l = lines[i+1].strip()
            
            if curr == '' and prev.startswith('///') and next_l.startswith('///'):
                continue
        
        # Also handle "empty line after doc comment" before function
        # /// Doc
        # 
        # pub fn
        if i > 0 and i < len(lines) - 1:
            curr = lines[i].strip()
            prev = lines[i-1].strip()
            next_l = lines[i+1].strip()
            if curr == '' and prev.startswith('///') and not next_l.startswith('///') and next_l != '':
                continue

        # Handle Outer Attribute empty line
        # /// Doc
        # 
        # /// Doc 2 (error case?)
        # No, clippy says "empty line after outer attribute"
        # /// Doc
        # 
        # pub fn
        # This is actually allowed usually?
        # `#[pyclass]`
        # 
        # pub struct
        # This triggers it.
        
        new_lines.append(lines[i])
    
    with open(fp, 'w', encoding='utf-8') as f:
        f.writelines(new_lines)

strict_remove_empty_doc_line('src/dataframe/manipulation.rs')
strict_remove_empty_doc_line('src/io/csv.rs')
strict_remove_empty_doc_line('src/series/mod.rs')
strict_remove_empty_doc_line('src/lazy.rs')
strict_remove_empty_doc_line('src/python_bindings.rs')

print("Fixing imports and doc comments v2.")
