# Go

A collection of Go projects organized by topic. This README serves as a quick-reference guide for working with Go modules, workspaces, and common workflows.

## Current Projects

```
go/
└── dsa/
    └── invert_binary_tree/   # Binary tree inversion algorithm
```

---

## Getting Started

### Install Go

Download from [go.dev/dl](https://go.dev/dl/) or use a package manager:

```bash
# Arch Linux
sudo pacman -S go

# macOS
brew install go

# Verify installation
go version
```

### Environment

Go uses two key environment variables:

| Variable   | Purpose                                      | Default              |
|------------|----------------------------------------------|----------------------|
| `GOPATH`   | Where downloaded modules and binaries live   | `~/go`               |
| `GOROOT`   | Where Go itself is installed                 | Set by installer     |

Check your environment:

```bash
go env
go env GOPATH GOROOT
```

---

## Modules

A **module** is a collection of Go packages with a `go.mod` file at the root. Every Go project should be a module.

### Initialize a New Module

```bash
mkdir my-project && cd my-project
go mod init github.com/k-henry/my-project
```

This creates a `go.mod` file:

```
module github.com/k-henry/my-project

go 1.23
```

> **Convention:** Use your GitHub path as the module name (e.g., `github.com/k-henry/project`). This makes your module importable by others and keeps naming consistent.

### Create Your Entry Point

```bash
touch main.go
```

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
```

### Run and Build

```bash
go run .           # Compile and run without producing a binary
go build .         # Compile and produce a binary in the current directory
go install .       # Compile and install the binary to $GOPATH/bin
```

### Add Dependencies

```bash
go get github.com/some/package          # Add a dependency
go get github.com/some/package@v1.2.3   # Add a specific version
go get github.com/some/package@latest   # Update to latest
```

### Tidy Dependencies

```bash
go mod tidy
```

This does two things:
1. **Adds** any dependencies that are imported in your code but missing from `go.mod`
2. **Removes** any dependencies listed in `go.mod` that your code no longer imports

Run `go mod tidy` after adding/removing imports, before committing, and when `go.sum` gets out of sync.

### Key Module Files

| File       | Purpose                                                    | Commit? |
|------------|------------------------------------------------------------|---------|
| `go.mod`   | Declares module path, Go version, and direct dependencies  | Yes     |
| `go.sum`   | Checksums for all dependencies (direct and indirect)       | Yes     |

---

## Workspaces

Workspaces let you work on **multiple modules simultaneously** without publishing them. Useful when you have several related projects in one directory.

### Initialize a Workspace

From this `go/` directory:

```bash
go work init
```

This creates a `go.work` file. Then add modules to it:

```bash
go work use ./dsa/invert_binary_tree
go work use ./some-other-project
```

The resulting `go.work` file:

```
go 1.23

use (
    ./dsa/invert_binary_tree
    ./some-other-project
)
```

### When to Use Workspaces

- You have multiple modules in one repo that depend on each other
- You want to test local changes across modules without publishing
- You're working on a library and a consumer of that library at the same time

### Workspace Commands

```bash
go work init              # Create a go.work file
go work use ./path        # Add a module to the workspace
go work sync              # Sync workspace dependencies
go work edit              # Edit go.work programmatically
```

> **Note:** Don't commit `go.work` and `go.work.sum` unless the workspace is intentional for all contributors. Add them to `.gitignore` for personal use.

---

## Project Structure Conventions

### Single Binary

```
my-project/
├── go.mod
├── go.sum
├── main.go
└── internal/
    └── helpers.go
```

### Library Package

```
my-library/
├── go.mod
├── go.sum
├── library.go
└── library_test.go
```

### Larger Project

```
my-app/
├── go.mod
├── go.sum
├── cmd/
│   └── my-app/
│       └── main.go
├── internal/
│   ├── handler/
│   └── model/
├── pkg/             # (optional) publicly importable packages
└── api/
```

Key directories:
- `cmd/` — Entry points (one subdirectory per binary)
- `internal/` — Private packages (cannot be imported by other modules)
- `pkg/` — Public packages (importable by other modules)

---

## Testing

```bash
go test ./...              # Run all tests recursively
go test ./dsa/...          # Run tests in a subtree
go test -v ./...           # Verbose output
go test -run TestName ./   # Run a specific test
go test -count=1 ./...     # Disable test caching
go test -cover ./...       # Show coverage percentage
go test -bench=. ./...     # Run benchmarks
```

Test files live next to the code they test and end in `_test.go`:

```go
// math_test.go
package math

import "testing"

func TestAdd(t *testing.T) {
    got := Add(2, 3)
    if got != 5 {
        t.Errorf("Add(2, 3) = %d, want 5", got)
    }
}
```

---

## Formatting and Linting

```bash
go fmt ./...         # Format all Go files (canonical style)
go vet ./...         # Report suspicious constructs
```

`go fmt` is non-negotiable in Go — all Go code uses the same formatting. Run it before every commit.

---

## Common Commands Cheat Sheet

| Command                        | What It Does                                    |
|--------------------------------|-------------------------------------------------|
| `go mod init <module-path>`    | Initialize a new module                         |
| `go mod tidy`                  | Sync dependencies with imports                  |
| `go get <package>`             | Add or update a dependency                      |
| `go run .`                     | Compile and run                                 |
| `go build .`                   | Compile to binary                               |
| `go install .`                 | Install binary to `$GOPATH/bin`                 |
| `go test ./...`                | Run all tests                                   |
| `go fmt ./...`                 | Format code                                     |
| `go vet ./...`                 | Static analysis                                 |
| `go doc <package>`             | View package documentation                      |
| `go work init`                 | Create a workspace                              |
| `go work use ./path`           | Add module to workspace                         |
| `go clean -cache`              | Clear build cache                               |
| `go env`                       | Print Go environment variables                  |

---

## Useful References

| Resource | Link |
|----------|------|
| Go Documentation | [go.dev/doc](https://go.dev/doc/) |
| Effective Go | [go.dev/doc/effective_go](https://go.dev/doc/effective_go) |
| Go by Example | [gobyexample.com](https://gobyexample.com/) |
| Go Tour (interactive) | [go.dev/tour](https://go.dev/tour/) |
| Standard Library | [pkg.go.dev/std](https://pkg.go.dev/std) |
| Go Module Reference | [go.dev/ref/mod](https://go.dev/ref/mod) |
| Go Playground | [go.dev/play](https://go.dev/play/) |
| Go Wiki | [go.dev/wiki](https://go.dev/wiki/) |
| Go Project Layout | [github.com/golang-standards/project-layout](https://github.com/golang-standards/project-layout) |
