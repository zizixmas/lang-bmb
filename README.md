# BMB - Bare-Metal-Banter

A contract-verified systems programming language.

Hard to write. Hard to get wrong.
And that's what AI prefers.

## Why BMB?

Traditional languages prioritize human convenience—readable syntax, flexible conventions, implicit behaviors. The cost: ambiguity, guesswork, runtime surprises.

BMB takes a different approach. Contracts are mandatory. Specifications are explicit. Invariants are verified at compile time.

**BMB unifies code, documentation, and tests:**

```bmb
fn binary_search(arr: &[i64], target: i64) -> i64
  pre is_sorted(arr)
  post ret == -1 or (0 <= ret and ret < len(arr))
  post ret != -1 implies arr[ret] == target
  post ret == -1 implies forall i: 0..len(arr). arr[i] != target
= {
    var lo = 0;
    var hi = len(arr) - 1;
    while lo <= hi
      invariant 0 <= lo and hi < len(arr)
      invariant forall i: 0..lo. arr[i] < target
      invariant forall i: (hi+1)..len(arr). arr[i] > target
    {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] == target then mid
        else if arr[mid] < target then { lo = mid + 1; }
        else { hi = mid - 1; }
    };
    -1
};
```

## Priorities (P0)

| Priority | Principle |
|----------|-----------|
| P0 | **Performance** — No syntax that constrains optimization. Target: exceed C/Rust. |
| P0 | **Correctness** — If it can be verified at compile time, it must be. |

## Quick Start

```bash
cargo build --release
bmb run examples/hello.bmb
bmb check examples/simple.bmb
bmb verify examples/verify.bmb   # requires Z3
bmb build examples/hello.bmb     # requires LLVM
bmb repl
```

## Current Version: v0.100

**P0 Performance Achieved**: BMB matches C -O3 performance.

```
Benchmark: fib(45)
C (-O3):   1.65s (100%)
BMB:       1.63s (99%)
```

See [ROADMAP.md](docs/ROADMAP.md) for detailed progress and planned features.

## Project Structure

```
lang-bmb/
├── bmb/           # Compiler (Rust)
├── bootstrap/     # Self-hosted compiler (BMB)
├── stdlib/        # Standard library
├── ecosystem/     # Tools, editor support
└── docs/          # Specification, roadmap
```

## Requirements

- Rust 1.70+
- Z3 (verification)
- LLVM 18+ (native codegen, optional)

## Documentation

- [SPECIFICATION.md](docs/SPECIFICATION.md)
- [ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [ROADMAP.md](docs/ROADMAP.md)

## License

MIT
