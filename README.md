# code

A personal monorepo of learning projects and small programs across several systems languages.

## Layout

```
.
├── c/      # C programs (Beej's guide, DSA problems)
├── cpp/    # C++ programs (DSA problems)
├── go/     # Go projects (DSA, Go by Example)
├── rust/   # Rust projects (The Rust Programming Language, DSA, SQL, misc.)
└── zig/    # Zig programs (DSA problems)
```

Each top-level language directory is organized by topic. `dsa/` subdirectories contain the same problems (invert binary tree, max depth, reverse linked list) re-implemented in each language for comparison.

Language-specific workflow notes and cheat sheets:

- [`go/README.md`](go/README.md) — modules, workspaces, testing, formatting
- [`rust/README.md`](rust/README.md) — Cargo, crates, workspaces, testing, formatting
- [`rust/sql_and_rust/README.md`](rust/sql_and_rust/README.md) — sqlx + Postgres walkthrough

## Requirements

Install whichever toolchains you need:

| Language | Toolchain        |
|----------|------------------|
| C        | `gcc` or `clang` |
| C++      | `g++` or `clang++` |
| Go       | `go`             |
| Rust     | `rustup` (stable) |
| Zig      | `zig`            |

## Conventions

- The root `.gitignore` covers build artifacts for all five languages (`target/`, `zig-cache/`, `*.o`, etc.) so individual projects don't need their own.
- `.env` files are ignored globally; commit `.env.example` if defaults are needed.
- `Cargo.lock` is committed for binaries (per Cargo's guidance for applications).
