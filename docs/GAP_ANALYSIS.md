# BMB Self-Hosting Gap Analysis

**Version**: v0.18.1 → v0.19
**Date**: 2026-01-04
**Status**: Critical Assessment

## Executive Summary

This document provides a critical analysis of the requirements for BMB self-hosting (v0.19 Mirror). The assessment reveals significant gaps between current implementation and self-hosting requirements that necessitate a revised roadmap.

**Key Finding**: The original v0.19 "Mirror" plan claiming full self-hosting is unrealistic. Core compiler features are incomplete, and true self-hosting should be targeted for v0.22+.

## Current Implementation Status

### Rust Compiler (13,673 LOC)

| Module | LOC | Status | Notes |
|--------|-----|--------|-------|
| types/mod.rs | 1,424 | Complete | Generics, methods working |
| codegen/wasm_text.rs | 1,115 | Partial | String constants TODO |
| lsp/mod.rs | 1,113 | Complete | Diagnostics, hover, goto |
| interp/eval.rs | 1,003 | Complete | Full expression evaluation |
| main.rs | 971 | Complete | CLI interface |
| mir/lower.rs | 817 | **Incomplete** | 8 major TODOs |
| build/mod.rs | 729 | Complete | Build orchestration |
| codegen/llvm.rs | 670 | Complete | inkwell bindings |
| verify/contract.rs | 635 | Complete | Z3 integration |

### Bootstrap Components (BMB, ~200KB total)

| File | Size | Status | Coverage |
|------|------|--------|----------|
| compiler.bmb | 42KB | Complete | Full pipeline demo |
| llvm_ir.bmb | 35KB | Complete | 93 tests |
| pipeline.bmb | 25KB | Complete | Source→MIR |
| parser_test.bmb | 25KB | Complete | 15 categories |
| lowering.bmb | 25KB | Partial | No struct/enum |
| parser.bmb | 22KB | Complete | Syntax validation |
| parser_ast.bmb | 21KB | Complete | S-expression output |
| mir.bmb | 18KB | Partial | Basic instructions |
| types.bmb | 15KB | Partial | Primitive types only |
| lexer.bmb | 8KB | Complete | All tokens |

## Critical Gaps

### 1. MIR Lowering (BLOCKING)

Located in `bmb/src/mir/lower.rs`:

```rust
// Lines 357-370: Struct/Enum NOT implemented
Expr::StructInit { .. } => {
    // TODO: Implement struct initialization in MIR
    Operand::Constant(Constant::Unit)
}

Expr::FieldAccess { .. } => {
    // TODO: Implement field access in MIR
    Operand::Constant(Constant::Unit)
}

Expr::EnumVariant { .. } => {
    // TODO: Implement enum variant construction in MIR
    Operand::Constant(Constant::Unit)
}
```

**Impact**: Cannot compile programs using:
- `Option<T>` / `Result<T,E>` enums
- Struct-based data structures (AST nodes, tokens)
- Pattern matching on enum variants
- The entire stdlib packages

### 2. Additional MIR Gaps

| Feature | Line | Status | Impact |
|---------|------|--------|--------|
| Pattern Matching | 468 | Stub only | Enum dispatch broken |
| Array Literals | 496 | Returns Unit | No arrays |
| Array Indexing | 504 | Returns 0 | No array access |
| Method Dispatch | 514 | Returns 0 | .unwrap_or() broken |

### 3. Language Features Missing

| Feature | Parser | Type Check | MIR | Codegen | Required For |
|---------|--------|------------|-----|---------|--------------|
| Closures | ❌ | ❌ | ❌ | ❌ | map, filter, fold |
| Traits | ❌ | ❌ | ❌ | ❌ | Abstraction |
| impl blocks | ❌ | ❌ | ❌ | ❌ | Methods |
| FFI linking | ⚠️ | ⚠️ | ❌ | ❌ | System calls |
| Dynamic alloc | ❌ | ❌ | ❌ | ❌ | Vec, String heap |

### 4. Bootstrap Limitations

The bootstrap files are designed for **demonstration**, not actual compilation:

- Only primitive types (i64, bool) supported
- No struct/enum handling
- No generics
- No module imports
- Output TEXT LLVM IR (not actual LLVM API calls)

## Self-Hosting Requirements Analysis

Based on research of successful self-hosting compilers (Zig, Rust, Go):

### Stage Model Required

```
Stage 0: Pre-compiled Rust compiler
Stage 1: Build BMB compiler using Stage 0
Stage 2: Build BMB compiler using Stage 1
Stage 3: Verify Stage 2 == Stage 3 (binary identical)
```

### Minimum Feature Requirements

| Feature | Rust Status | Bootstrap Status | Required |
|---------|-------------|------------------|----------|
| Functions | ✅ | ✅ | ✅ |
| Control flow | ✅ | ✅ | ✅ |
| Structs | ✅ Parse, ❌ MIR | ❌ | ✅ |
| Enums | ✅ Parse, ❌ MIR | ❌ | ✅ |
| Generics | ✅ | ❌ | ✅ |
| Pattern Match | ⚠️ Partial | ❌ | ✅ |
| Strings (heap) | ⚠️ Primitive | ⚠️ | ✅ |
| Arrays/Vec | ❌ | ❌ | ✅ |
| Closures | ❌ | ❌ | ✅ |
| Traits | ❌ | ❌ | ✅ |

### Effort Estimate

| Component | Effort (LOC) | Complexity | Priority |
|-----------|--------------|------------|----------|
| Struct MIR | ~200-300 | Medium | P0 |
| Enum MIR | ~300-400 | High | P0 |
| Pattern Match | ~400-500 | High | P0 |
| Array MIR | ~200-300 | Medium | P1 |
| Method Dispatch | ~200-300 | Medium | P1 |
| Closures | ~500-700 | High | P2 |
| Traits | ~800-1000 | Very High | P2 |

## Revised Roadmap Proposal

### Phase 1: MIR Completion (v0.19.0-3)

**v0.19.0 - Struct Support**
- Add `MirInst::StructInit` and `MirInst::FieldAccess`
- Implement struct MIR lowering
- Add LLVM codegen for structs
- Test: Compile `Pair<A,B>`, `Range` structs

**v0.19.1 - Enum Support**
- Add `MirInst::EnumVariant`
- Implement enum discriminant handling
- Add switch terminator for pattern matching
- Test: Compile `Option<T>`, `Result<T,E>`

**v0.19.2 - Pattern Matching**
- Full pattern matching compilation
- Nested patterns
- Guard clauses
- Test: Match on Option/Result variants

**v0.19.3 - Array Support**
- Array literal lowering
- Array indexing with bounds check
- Fixed-size arrays
- Test: Compile array operations

### Phase 2: Method Dispatch (v0.19.4-5)

**v0.19.4 - Known Type Methods**
- Method call MIR lowering
- Receiver type resolution
- Test: `.is_some()`, `.unwrap_or()`

**v0.19.5 - Integration Testing**
- Compile all packages/bmb-* with LLVM
- Validate generated IR
- Benchmark basic operations

### Phase 3: Language Extensions (v0.20.x)

**v0.20.0 - Closures**
- Lambda syntax parsing
- Capture semantics
- Closure type inference

**v0.20.1 - Trait Foundation**
- trait keyword parsing
- impl block parsing
- Basic trait resolution

**v0.20.2 - FFI Enhancement**
- extern "C" linking
- ABI handling
- C library interop

### Phase 4: Bootstrap Enhancement (v0.21.x)

**v0.21.0 - Bootstrap Struct/Enum**
- Extend bootstrap lowering for structs
- Extend bootstrap lowering for enums

**v0.21.1 - Bootstrap Integration**
- Test bootstrap against simple programs
- Validate MIR text output

### Phase 5: Self-Hosting Attempt (v0.22.x)

**v0.22.0 - Stage 1 Compilation**
- Compile BMB compiler source using Rust compiler
- Generate BMB binary

**v0.22.1 - Stage 2 Verification**
- Compile BMB compiler using Stage 1
- Compare outputs

## Recommendations

1. **Rename v0.19**: Change from "Mirror (Self-Hosting)" to "MIR Completion"
2. **Defer Self-Hosting**: Target v0.22+ for actual self-hosting
3. **Prioritize MIR**: Focus on completing Rust compiler MIR before bootstrap
4. **Incremental Testing**: Add integration tests at each phase
5. **Documentation**: Update ROADMAP.md with realistic estimates

## Conclusion

True self-hosting requires:
- ~2,500-3,500 LOC changes to Rust compiler
- ~1,000+ LOC bootstrap enhancements
- Complete MIR lowering for structs/enums/patterns
- Working closures and traits

The original v0.19 "Mirror" plan should be re-scoped to focus on MIR completion, with self-hosting targeted for v0.22+.

---

**Next Actions**:
1. [ ] Update ROADMAP.md with revised phases
2. [ ] Implement v0.19.0 Struct MIR support
3. [ ] Add integration tests for MIR lowering
4. [ ] Create tracking issues for each gap
