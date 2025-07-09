import sys
sys.path.insert(0, 'D:/Rust/Velox/velox/target/wheels')

try:
    import veloxx
    print(f"veloxx.__file__: {veloxx.__file__}")
    print(f"veloxx contents: {dir(veloxx)}")
except ImportError as e:
    print(f"ImportError: {e}")
except AttributeError as e:
    print(f"AttributeError: {e}")
except Exception as e:
    print(f"An unexpected error occurred: {e}")