@echo off
REM ========================================
REM GitHub Actions CI Validation Script
REM ========================================
REM This script replicates the exact GitHub Actions CI workflow locally
REM Run this before pushing to ensure CI will pass

setlocal enabledelayedexpansion

echo.
echo ==========================================
echo 🔄 GitHub Actions CI Validation
echo ==========================================
echo.

REM Start timer
set START_TIME=%time%

echo 📋 Running exact GitHub Actions workflow steps...
echo.

REM ========================================
REM Test Suite Job (from .github/workflows/ci.yml)
REM ========================================
echo 🧪 TEST SUITE JOB
echo ==========================================

echo 🔍 Check Formatting
echo Command: cargo fmt --all -- --check
echo ----------------------------------------
cargo fmt --all -- --check
if !errorlevel! equ 0 (
    echo ✅ Check Formatting: PASSED
) else (
    echo ❌ Check Formatting: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

echo 🔍 Lint with Clippy
echo Command: cargo clippy --all-targets --all-features -- -D warnings
echo ----------------------------------------
cargo clippy --all-targets --all-features -- -D warnings
if !errorlevel! equ 0 (
    echo ✅ Lint with Clippy: PASSED
) else (
    echo ❌ Lint with Clippy: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

echo 🔍 Run Tests
echo Command: cargo test --verbose
echo ----------------------------------------
cargo test --verbose
if !errorlevel! equ 0 (
    echo ✅ Run Tests: PASSED
) else (
    echo ❌ Run Tests: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

echo 🔍 Test Documentation
echo Command: cargo test --doc --verbose
echo ----------------------------------------
cargo test --doc --verbose
if !errorlevel! equ 0 (
    echo ✅ Test Documentation: PASSED
) else (
    echo ❌ Test Documentation: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

echo 🔍 Build Release
echo Command: cargo build --release --verbose
echo ----------------------------------------
cargo build --release --verbose
if !errorlevel! equ 0 (
    echo ✅ Build Release: PASSED
) else (
    echo ❌ Build Release: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

REM ========================================
REM Security Audit Job (from .github/workflows/ci.yml)
REM ========================================
echo 🔒 SECURITY AUDIT JOB
echo ==========================================

REM Check if cargo-audit is installed
where cargo-audit >nul 2>&1
if !errorlevel! neq 0 (
    echo ⚠️ Installing cargo-audit...
    cargo install cargo-audit
)

echo 🔍 Security Audit
echo Command: cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2024-0384 --ignore RUSTSEC-2024-0436
echo ----------------------------------------
cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2024-0384 --ignore RUSTSEC-2024-0436
if !errorlevel! equ 0 (
    echo ✅ Security Audit: PASSED
) else (
    echo ❌ Security Audit: FAILED
    echo This step will cause CI to fail!
    goto :failure
)
echo.

REM ========================================
REM Summary
REM ========================================
echo ==========================================
echo 📊 CI VALIDATION SUMMARY
echo ==========================================

set END_TIME=%time%
echo Duration: %START_TIME% to %END_TIME%
echo.

echo 🎉 ALL CI STEPS PASSED!
echo Your changes are ready to push and will pass GitHub Actions CI.
echo.
echo 💡 Next steps:
echo    - git push
echo    - Monitor the GitHub Actions workflow
echo.
echo 🔗 Workflow URLs:
echo    - Test Suite: Will appear in GitHub Actions after push
echo    - Security Audit: Will appear in GitHub Actions after push

exit /b 0

:failure
echo.
echo ==========================================
echo ❌ CI VALIDATION FAILED
echo ==========================================
echo.
echo One or more CI steps failed. Fix the issues above before pushing.
echo.
echo 💡 Common fixes:
echo    - Run: cargo fmt --all
echo    - Fix clippy warnings
echo    - Fix failing tests
echo    - Check security audit issues
echo.
exit /b 1