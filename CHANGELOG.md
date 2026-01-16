# Changelog

All notable changes to BMB (Bare-Metal-Banter) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.50.13] - 2026-01-16

### Fixed

- **Bootstrap LLVM IR variable scoping bug**: Function parameters were incorrectly renamed with block suffixes (e.g., `%d` → `%d_b2`) in nested else branches, causing undefined variable errors in generated LLVM IR.

### Changed

- Added `params` parameter to all `lower_*_sb` functions in bootstrap LLVM IR generator
- New `extract_param_names` helper extracts parameter names from signature for scoping checks
- `lower_var_sb` now uses `is_param()` to preserve original parameter names across all blocks

### Known Issues

- Stage 2 self-compilation still fails due to stack overflow when processing 30K+ line bootstrap file (pre-existing issue, tracked as v0.46 blocker)

## [0.50.12] - 2026-01-16

### Fixed

- **Critical performance bug**: LLVM optimization passes were not being run on generated IR, causing 5x slower native code than C. Now runs `default<O2>` or `default<O3>` passes based on optimization level.

### Performance

- **Native code benchmark**: fibonacci(40) improved from 5.15x slower to 2.0x slower than C (gcc -O3). The remaining gap is due to GCC's more aggressive loop unrolling.

### Changed

- Migrated all benchmark files in `ecosystem/benchmark-bmb/` to v0.32 syntax

## [0.50.11] - 2026-01-16

### Security

- **Cyclic type alias detection**: Added DFS-based cycle detection to prevent DoS via infinite recursion in type resolution. Circular definitions like `type A = B; type B = A;` now produce clear error messages.
- **Duplicate function warning**: Compiler now warns when a function is defined multiple times with the same name. Later definitions silently override earlier ones (warning helps catch copy-paste errors).

### Changed

- Extended `type_aliases` HashMap to track definition spans for better error reporting
- Added `function_spans` tracking to TypeChecker for duplicate detection

### Tests

- Added 7 new integration tests for type alias cycles and duplicate function detection

## [0.50.10] - 2026-01-16

### Security

- Completed Security Audit Phase 3: Penetration testing
- Documented all P0/P1 security findings in SECURITY_AUDIT.md

## [0.50.9] - 2026-01-15

### Documentation

- Critical benchmark review and honest status assessment
- Updated roadmap with verification results

## [0.50.8] - 2026-01-15

### Changed

- Bootstrap if-else refactoring for reduced parser complexity
- Simplified parser grammar to avoid stack overflow issues

## [0.50.6] - 2026-01-14

### Added

- **Type alias syntax**: `type Name = TargetType;` with generic parameter support
- **Refinement type aliases**: `type NonZero = i64 where self != 0;`
- Type alias resolution in type checker

## [0.50.5] - 2026-01-14

### Added

- Expanded integration test suite
- Fixed stdlib constants and type definitions

## [0.50.4] - 2026-01-14

### Fixed

- Stdlib contract syntax errors in multiple modules

## [0.50.3] - 2026-01-13

### Added

- Comprehensive integration test suite (65+ tests)
- Test infrastructure for error cases and warning detection

## [0.50.1] - 2026-01-13

### Fixed

- Stdlib postcondition syntax issues
- Bootstrap parser integer/keyword collision bugs

### Documentation

- Documented bootstrap compiler bottlenecks

## [0.50.0] - 2026-01-12

### Added

- Security Audit Phase 1: Automated security checks
- Security Audit Phase 2: Unsafe code review
- Critical review and honest project status assessment

### Changed

- v0.32 syntax migration completed for bootstrap compiler

## [0.45.0] - 2025-12-XX

### Added

- Multi-type REPL support
- Lint command with `--strict` flag for treating warnings as errors
- Enhanced warning system

## [0.32.0] - 2025-XX-XX

### Changed

- **Breaking**: New if-else syntax: `if cond { then } else { else }` (Rust-style braces)
- **Breaking**: Comments now use `//` (double-slash), `--` still supported for compatibility
- Added shift operators: `<<` (left shift), `>>` (right shift)
- Added symbolic logical operators: `&&`, `||`, `!` as alternatives to `and`, `or`, `not`

## [0.25.0] - 2025-XX-XX

### Added

- AI Query System (`bmb index`, `bmb q`)
- `.bmb/` project folder structure
- Symbol indexing for functions, types, and contracts

---

For migration guides and detailed release notes, see [docs/ROADMAP.md](docs/ROADMAP.md).
