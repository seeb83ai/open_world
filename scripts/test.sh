#!/bin/bash

set -e

echo "======================================"
echo "Open World - Full Test Suite"
echo "======================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 1. Unit Tests
echo -e "${BLUE}[1/4]${NC} Running unit tests..."
cargo test --lib
echo -e "${GREEN}✓ Unit tests passed${NC}"
echo ""

# 2. Integration Tests
echo -e "${BLUE}[2/4]${NC} Running integration tests..."
cargo test --test '*'
echo -e "${GREEN}✓ Integration tests passed${NC}"
echo ""

# 3. Code Quality Checks
echo -e "${BLUE}[3/4]${NC} Running code quality checks..."
cargo clippy -- -D warnings
echo -e "${GREEN}✓ Clippy checks passed${NC}"
echo ""

# 4. Format Check
echo -e "${BLUE}[4/4]${NC} Checking code formatting..."
cargo fmt -- --check
echo -e "${GREEN}✓ Format check passed${NC}"
echo ""

echo -e "${GREEN}======================================"
echo "All tests passed! ✓"
echo "======================================${NC}"
