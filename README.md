# code

A personal monorepo of learning projects and small programs across Go, Rust, and TypeScript.

## Layout

```
.
├── go/         # Go projects (DSA, Go by Example, LeetCode)
├── rust/       # Rust projects (The Rust Programming Language, DSA, SQL, misc.)
└── typescript/ # TypeScript projects (DSA)
```

Each top-level language directory is organized by topic. `dsa/` subdirectories contain the same problems (invert binary tree, max depth, reverse linked list) re-implemented in each language for comparison.

Language-specific workflow notes and cheat sheets:

- [`go/README.md`](go/README.md) — modules, workspaces, testing, formatting
- [`rust/README.md`](rust/README.md) — Cargo, crates, workspaces, testing, formatting
- [`rust/sql_and_rust/README.md`](rust/sql_and_rust/README.md) — sqlx + Postgres walkthrough

## Requirements

Install whichever toolchains you need:

| Language   | Toolchain         |
|------------|-------------------|
| Go         | `go`              |
| Rust       | `rustup` (stable) |
| TypeScript | `node` + `npm`    |

## Conventions

- The root `.gitignore` covers build artifacts for Go, Rust, and TypeScript so individual projects don't need their own.
- `.env` files are ignored globally; commit `.env.example` if defaults are needed.
- `Cargo.lock` is committed for binaries (per Cargo's guidance for applications).
- `package-lock.json` is committed (deterministic installs for applications).
