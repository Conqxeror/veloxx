#!/bin/bash

# ========================================
# GitHub Actions CI Validation Script
# ========================================
# This script replicates the exact GitHub Actions CI workflow locally
# Run this before pushing to ensure CI will pass

set -e  # Exit on error

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo
echo "=========================================="
echo "🔄 GitHub Actions CI Validation"
echo "=========================================="
echo

# Function to run a test step exactly as in CI
run_ci_step() {
    local step_name="$1"
    local step_command="$2"
    
    echo -e "${BLUE}🔍 $step_name${NC}"
    echo "Command: $step_command"
    echo "----------------------------------------"
    
    if eval "$step_command"; then
        echo -e "${GREEN}✅ $step_name: PASSED${NC}"
        echo
        return 0
    else
        echo -e "${RED}❌ $step_name: FAILED${NC}"
        echo -e "${RED}This step will cause CI to fail!${NC}"
        echo
        return 1
    fi
}

# Start timer
START_TIME=$(date +%s)

echo -e "${BLUE}📋 Running exact GitHub Actions workflow steps...${NC}"
echo

# ========================================
# Test Suite Job (from .github/workflows/ci.yml)
# ========================================
echo -e "${BLUE}🧪 TEST SUITE JOB${NC}"
echo "=========================================="

run_ci_step "Check Formatting" "cargo fmt --all -- --check"
run_ci_step "Lint with Clippy" "cargo clippy --all-targets --all-features -- -D warnings"
run_ci_step "Run Tests" "cargo test --verbose"
run_ci_step "Test Documentation" "cargo test --doc --verbose"
run_ci_step "Build Release" "cargo build --release --verbose"

echo

# ========================================
# Security Audit Job (from .github/workflows/ci.yml)
# ========================================
echo -e "${BLUE}🔒 SECURITY AUDIT JOB${NC}"
echo "=========================================="

# Check if cargo-audit is installed
if ! command -v cargo-audit >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️ Installing cargo-audit...${NC}"
    cargo install cargo-audit
fi

run_ci_step "Security Audit" "cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2024-0384 --ignore RUSTSEC-2024-0436"

echo

# ========================================
# Summary
# ========================================
echo "=========================================="
echo "📊 CI VALIDATION SUMMARY"
echo "=========================================="

# Calculate end time
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "Duration: ${DURATION}s"
echo

echo -e "${GREEN}🎉 ALL CI STEPS PASSED!${NC}"
echo -e "${GREEN}Your changes are ready to push and will pass GitHub Actions CI.${NC}"
echo
echo -e "${BLUE}💡 Next steps:${NC}"
echo "   - git push"
echo "   - Monitor the GitHub Actions workflow"
echo
echo -e "${BLUE}🔗 Workflow URLs:${NC}"
echo "   - Test Suite: Will appear in GitHub Actions after push"
echo "   - Security Audit: Will appear in GitHub Actions after push"