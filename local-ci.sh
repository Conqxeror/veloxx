#!/bin/bash

# ========================================
# Velox Local CI/CD Testing Script
# ========================================
# This script replicates essential GitHub workflow checks locally
# Run this before pushing to catch issues early

set -e  # Exit on error (can be overridden for individual tests)

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Initialize counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Start timer
START_TIME=$(date +%s)

echo
echo "=========================================="
echo "🚀 Velox Local CI/CD Testing Suite"
echo "=========================================="
echo

echo -e "${BLUE}📋 Starting comprehensive local testing...${NC}"
echo

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    local fix_command="$3"
    
    ((TOTAL_TESTS++))
    echo -n "🔍 $test_name... "
    
    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED${NC}"
        ((PASSED_TESTS++))
        return 0
    else
        echo -e "${RED}❌ FAILED${NC}"
        if [ -n "$fix_command" ]; then
            echo "   Fix with: $fix_command"
        fi
        ((FAILED_TESTS++))
        return 1
    fi
}

# Function to run optional test (won't fail the script)
run_optional_test() {
    local test_name="$1"
    local test_command="$2"
    local install_command="$3"
    
    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ $test_name: PASSED${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️ $test_name: SKIPPED${NC}"
        if [ -n "$install_command" ]; then
            echo "   Install with: $install_command"
        fi
        return 1
    fi
}

# ========================================
# 1. RUST CORE TESTS
# ========================================
echo -e "${BLUE}🦀 RUST CORE TESTS${NC}"
echo "----------------------------------------"

run_test "Rust formatting" "cargo fmt --all -- --check" "cargo fmt --all"
run_test "Clippy lints" "cargo clippy --all-targets --all-features -- -D warnings" "cargo clippy --all-targets --all-features -- -D warnings"
run_test "Unit tests" "cargo test --verbose" "cargo test --verbose"
run_test "Doc tests" "cargo test --doc --verbose" "cargo test --doc --verbose"
run_test "Feature builds" "cargo build --verbose && cargo build --features python --verbose && cargo build --features wasm --verbose" "Check individual feature builds"

echo

# ========================================
# 2. SECURITY AUDIT
# ========================================
echo -e "${BLUE}🔒 SECURITY AUDIT${NC}"
echo "----------------------------------------"

if command -v cargo-audit >/dev/null 2>&1; then
    run_test "Security audit" "cargo audit" "cargo audit"
else
    echo -e "${YELLOW}⚠️ Security audit: SKIPPED (cargo-audit not installed)${NC}"
    echo "   Install with: cargo install cargo-audit"
fi

echo

# ========================================
# 3. PYTHON BINDINGS (if available)
# ========================================
echo -e "${BLUE}🐍 PYTHON BINDINGS${NC}"
echo "----------------------------------------"

if command -v python3 >/dev/null 2>&1 || command -v python >/dev/null 2>&1; then
    PYTHON_CMD=$(command -v python3 2>/dev/null || command -v python)
    echo "🔍 Checking Python environment..."
    
    # Check if virtual environment exists
    if [ -d "venv" ] && [ -f "venv/bin/activate" ]; then
        echo "📦 Using existing virtual environment..."
        source venv/bin/activate
    else
        echo "📦 Creating virtual environment..."
        $PYTHON_CMD -m venv venv
        source venv/bin/activate
        pip install --upgrade pip >/dev/null 2>&1
        pip install -r requirements.txt >/dev/null 2>&1
    fi
    
    run_test "Python build" "maturin build --features python" "maturin build --features python"
    
    if [ $? -eq 0 ]; then
        run_test "Python import" "$PYTHON_CMD -c 'import veloxx; print(\"Python bindings work!\")'" ""
        
        # Run Python tests if they exist
        if [ -d "tests/python" ]; then
            run_test "Python tests" "$PYTHON_CMD -m pytest tests/python/ -v" "python -m pytest tests/python/ -v"
        fi
    fi
    
    deactivate 2>/dev/null || true
else
    echo -e "${YELLOW}⚠️ Python tests: SKIPPED (Python not found)${NC}"
fi

echo

# ========================================
# 4. WASM BINDINGS (if available)
# ========================================
echo -e "${BLUE}🕸️ WASM BINDINGS${NC}"
echo "----------------------------------------"

if command -v wasm-pack >/dev/null 2>&1; then
    run_test "WASM build" "wasm-pack build --target web --out-dir pkg --features wasm" "wasm-pack build --target web --out-dir pkg --features wasm"
    
    if [ $? -eq 0 ] && command -v node >/dev/null 2>&1; then
        echo "📦 Installing Node.js dependencies..."
        npm install >/dev/null 2>&1
        run_test "WASM tests" "npm test" "npm test"
    elif [ ! command -v node >/dev/null 2>&1 ]; then
        echo -e "${YELLOW}⚠️ WASM tests: SKIPPED (Node.js not found)${NC}"
    fi
else
    echo -e "${YELLOW}⚠️ WASM tests: SKIPPED (wasm-pack not installed)${NC}"
    echo "   Install from: https://rustwasm.github.io/wasm-pack/installer/"
fi

echo

# ========================================
# 5. DOCUMENTATION
# ========================================
echo -e "${BLUE}📚 DOCUMENTATION${NC}"
echo "----------------------------------------"

run_test "Documentation build" "cargo doc --all-features --no-deps" "cargo doc --all-features --no-deps"

echo

# ========================================
# 6. RELEASE BUILD
# ========================================
echo -e "${BLUE}🎯 RELEASE BUILD${NC}"
echo "----------------------------------------"

run_test "Release build" "cargo build --release" "cargo build --release"

# Show binary size if available
if [ -f "target/release/veloxx" ]; then
    SIZE=$(stat -f%z "target/release/veloxx" 2>/dev/null || stat -c%s "target/release/veloxx" 2>/dev/null || echo "unknown")
    echo "   Binary size: $SIZE bytes"
fi

echo

# ========================================
# SUMMARY
# ========================================
echo "=========================================="
echo "📊 TEST SUMMARY"
echo "=========================================="

# Calculate end time
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "Total tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
echo -e "${RED}Failed: $FAILED_TESTS${NC}"

SKIPPED_TESTS=$((TOTAL_TESTS - PASSED_TESTS - FAILED_TESTS))
if [ $SKIPPED_TESTS -gt 0 ]; then
    echo -e "${YELLOW}Skipped: $SKIPPED_TESTS${NC}"
fi

echo "Duration: ${DURATION}s"
echo

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED! Ready to push.${NC}"
    echo
    echo -e "${BLUE}💡 Next steps:${NC}"
    echo "   - git add ."
    echo "   - git commit -m \"Your commit message\""
    echo "   - git push"
    exit 0
else
    echo -e "${RED}❌ SOME TESTS FAILED! Fix issues before pushing.${NC}"
    echo
    echo -e "${BLUE}💡 To fix issues:${NC}"
    echo "   - Review failed tests above"
    echo "   - Run individual commands to see detailed errors"
    echo "   - Fix issues and re-run this script"
    exit 1
fi