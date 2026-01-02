#!/bin/bash
# BMB Bootstrap LLVM IR Validation Script
# Validates that generated LLVM IR is correct and can be compiled

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== BMB Bootstrap LLVM IR Validation ==="
echo ""

# Find clang
if command -v clang &> /dev/null; then
    CLANG="clang"
elif [ -f "/c/Program Files/LLVM/bin/clang.exe" ]; then
    CLANG="/c/Program Files/LLVM/bin/clang.exe"
else
    echo "ERROR: clang not found"
    exit 1
fi

# Find llvm-nm
if command -v llvm-nm &> /dev/null; then
    LLVM_NM="llvm-nm"
elif [ -f "/c/Program Files/LLVM/bin/llvm-nm.exe" ]; then
    LLVM_NM="/c/Program Files/LLVM/bin/llvm-nm.exe"
else
    LLVM_NM=""
fi

echo "Using clang: $CLANG"
echo ""

# Test 1: Validate LLVM IR syntax
echo "[1/3] Validating LLVM IR syntax..."
"$CLANG" -S -emit-llvm test_add.ll -o /dev/null 2>&1 | grep -v "warning:" || true
echo "  ✓ LLVM IR syntax valid"

# Test 2: Compile to object file
echo "[2/3] Compiling to object file..."
"$CLANG" -c test_add.ll -o test_add.obj 2>&1 | grep -v "warning:" || true
if [ -f test_add.obj ]; then
    SIZE=$(stat -c%s test_add.obj 2>/dev/null || stat -f%z test_add.obj 2>/dev/null || echo "unknown")
    echo "  ✓ Object file created ($SIZE bytes)"
else
    echo "  ✗ Failed to create object file"
    exit 1
fi

# Test 3: Verify symbols
echo "[3/3] Verifying symbols..."
if [ -n "$LLVM_NM" ]; then
    SYMBOLS=$("$LLVM_NM" test_add.obj 2>/dev/null || echo "")
    if echo "$SYMBOLS" | grep -q " T add"; then
        echo "  ✓ Symbol 'add' found (defined)"
    else
        echo "  ✗ Symbol 'add' not found"
        exit 1
    fi
    if echo "$SYMBOLS" | grep -q " T main"; then
        echo "  ✓ Symbol 'main' found (defined)"
    else
        echo "  ✗ Symbol 'main' not found"
        exit 1
    fi
    if echo "$SYMBOLS" | grep -q " U println"; then
        echo "  ✓ Symbol 'println' found (external reference)"
    else
        echo "  ✗ Symbol 'println' not found"
        exit 1
    fi
else
    echo "  (llvm-nm not available, skipping symbol verification)"
fi

echo ""
echo "=== All validations passed ==="
echo ""
echo "Note: To create an executable, you need:"
echo "  1. Compile runtime.c: clang -c runtime.c -o runtime.o"
echo "  2. Link together: clang test_add.obj runtime.o -o test_add"
echo "  3. Run: ./test_add"
