# Contributing to Veloxx

We welcome contributions to Veloxx! To ensure a smooth and efficient collaboration, please follow these guidelines.

## Getting Started

1.  **Fork the repository:** Start by forking the Veloxx repository on GitHub.
2.  **Clone your fork:** Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/your-username/veloxx.git
    cd veloxx
    ```
3.  **Install Rust:** If you don't have Rust installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
4.  **Install `wasm-pack`:** For WebAssembly development, you'll need `wasm-pack`:
    ```bash
    cargo install wasm-pack
    ```
5.  **Install `maturin`:** For Python bindings, you'll need `maturin`:
    ```bash
    pip install maturin
    ```
6.  **Set up Python virtual environment:**
    ```bash
    python -m venv venv
    .\venv\Scripts\activate # On Windows
    source venv/bin/activate # On macOS/Linux
    pip install -r requirements.txt # If a requirements.txt exists
    ```

## Development Workflow

1.  **Create a new branch:** For each new feature or bug fix, create a new branch:
    ```bash
    git checkout -b feature/your-feature-name
    ```
    or
    ```bash
    git checkout -b bugfix/your-bug-fix-name
    ```
2.  **Make your changes:** Implement your feature or bug fix.
3.  **Run tests:** Before committing, ensure all tests pass. See the "Testing" section below.
4.  **Commit your changes:** Write clear and concise commit messages.
    ```bash
    git commit -m "feat: Add new feature" # or "fix: Fix bug"
    ```
5.  **Push to your fork:**
    ```bash
    git push origin feature/your-feature-name
    ```
6.  **Create a Pull Request (PR):** Open a pull request from your fork to the `main` branch of the Veloxx repository. Provide a detailed description of your changes.

## Testing

### Rust Unit and Integration Tests

To run the core Rust tests:

```bash
cargo test
```

### Python Bindings Tests

To run the Python binding tests (ensure your virtual environment is activated and `maturin develop` has been run):

```bash
pytest tests/python/test_veloxx.py
```

### WebAssembly Bindings Tests

To run the WebAssembly binding tests:

1.  Build the WebAssembly module:
    ```bash
    wasm-pack build --target web --out-dir pkg
    ```
2.  Install JavaScript dependencies:
    ```bash
    npm install --prefix pkg
    ```
3.  Run the tests:
    ```bash
    npm test --prefix pkg
    ```

## Code Style and Linting

- **Rust:** Follow the [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md). Use `cargo fmt` to format your code.
- **Python:** Adhere to [PEP 8](https://www.python.org/dev/peps/pep-0008/). Use a linter like `flake8` or `ruff`.

## Documentation

- Ensure all new public functions, structs, and enums are properly documented using Rustdoc (`///`).
- Update the `README.md` if your changes introduce new features or significantly alter existing ones.
- Add entries to `CHANGELOG.md` for new features, bug fixes, and breaking changes.

## License

By contributing to Veloxx, you agree that your contributions will be licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
