## Veloxx — Guidance for AI assistants (concise)

This file exists to help AI coding agents be immediately useful in the Veloxx codebase.
Focus on practical, discoverable facts about architecture, dev workflows and coding patterns.

1) Big-picture (quick)
   - Veloxx is a Rust-first high-performance data library (src/) exposing
     - Core API: `src/dataframe/*` and `src/series/*` (DataFrame / Series)
     - I/O: `src/io/{csv,json,arrow,mmap_csv}.rs`
     - Performance/optimizations: `src/performance/*` (SIMD, parallel, groupby, filter)
     - Language bindings: `src/python_bindings.rs`, `bindings/python/` and `src/wasm_bindings*` + `pkg/` (WASM output)

2) What to change and what to avoid
   - Performance modules are micro-optimized; small tweaks can have big regressions (look in `src/performance/*` and benches/).
   - When adding or changing code in `src/` add unit tests (Rust: tests/ and benches/, Python: tests/python/). Example tests: `tests/python/test_veloxx.py`.
   - Use feature flags in Cargo.toml for optional builds: `python`, `wasm`, `advanced_io`, `data_quality`, `ml`, `visualization`.

3) Developer workflows (commands you can run locally)
   - Rust unit + doc tests: `cargo test` and `cargo test --doc` (CI uses these). See scripts/validate-ci.* for local CI flow.
   - Python bindings (local dev): create/activate a venv then `maturin develop --features python` then `python -m pytest tests/python/ -v`.
     - Windows example (cmd): `venv\Scripts\activate && maturin develop --features python && python -m pytest tests/python/ -v`
   - WASM dev: `wasm-pack build --target web --out-dir pkg && npm test` (JS tests live in `tests/` and `pkg` run). See `tests/wasm_test.js` for usage.

4) CI / test signals
   - CI validates: `cargo test`, doc tests, `maturin` wheels / dev, wasm-pack builds, and npm tests — see `.github/workflows/` and `.github/WORKFLOWS.md`.
   - Benchmarks / performance checks are under `benches/` — use cargo bench locally when required.

5) Conventions & PR checklist (useful prompts for patch suggestions)
   - Follow the project's commit style (see CONTRIBUTING.md). Add tests for behavior changes and update docs (`docs/`, `README.md`) when APIs change.
   - Rust formatting and lints: `cargo fmt --all` and `cargo clippy --all-targets --all-features`.
   - Check for zero-warning builds on CI targets and compatibility with feature flags.

6) Useful files to inspect when reasoning about features
   - Implementation: `src/lib.rs`, `src/dataframe/mod.rs`, `src/series/mod.rs`
   - Bindings: `src/python_bindings.rs`, `bindings/python/` and `src/wasm_bindings.rs`
   - Tests: `tests/`, `tests/python/`, `benches/`, `examples/`
   - CI helpers & local checks: `scripts/local-ci.bat`, `scripts/validate-ci.sh`

If anything here is unclear or you'd like the guidance expanded around testing, performance benchmarks or bindings workflow, tell me which area and I'll iterate.
