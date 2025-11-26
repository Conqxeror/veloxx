
import os

file_path = 'src/python_bindings.rs'
with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

# The error is at:
#             PyJoinType::Right => crate::dataframe::join::JoinType::Right,
#             PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,
#     Outer,
#         };

# It seems I copy-pasted `Outer` into the enum variant or match arm incorrectly?
# Ah, the previous replace added `\n    Outer,` after the match arm? 
# Or `PyJoinType` enum definition.

# Let's fix the specific error block.
# It looks like:
# ...
# PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,
# Outer,
# };

# I should remove the stray `Outer,` line.

if '            PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,\n    Outer,' in content:
    content = content.replace('            PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,\n    Outer,', '            PyJoinType::Outer => crate::dataframe::join::JoinType::Outer,')

with open(file_path, 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed Python bindings syntax error.")
