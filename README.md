# bolero-test

This repository is a demo project for the [Soteria](https://github.com/soteria-tools/soteria) symbolic execution engine paired with the [Soteria fork of bolero](https://github.com/soteria-tools/bolero).

It contains a single function, `buggy_add`, with a deliberately planted bug: for the specific inputs `(12976, 14867)` it returns the wrong result. The standard unit test in `src/lib.rs` only checks `buggy_add(2, 2)` and passes without finding the bug. The bolero harness in `tests/test.rs` checks the function against all possible `u32` pairs:

- Running it in **fuzzing mode** will eventually find the bug, but may take a while since the triggering inputs are unlikely to be generated at random.
- Running it in **Soteria mode** finds the bug almost instantly via symbolic execution.

## Prerequisites

### 1. Install Soteria

> **Note:** The pre-built package only works on macOS with an ARM (Apple Silicon) architecture.

```sh
cargo install soteria --git https://github.com/soteria-tools/cargo-soteria.git
```

If you are running on a remote VM or a non-ARM machine, you will need to [install Soteria from source](https://github.com/soteria-tools/soteria#installing-from-source). The source installation instructions also cover installing the required `obol` frontend.

### 2. Install cargo-bolero

```sh
cargo install --git https://github.com/soteria-tools/bolero --rev bdcfbb840fd1022220d150fb9ced192af182ad06 cargo-bolero
```

If you are running on Linux, you also need to install the following dependencies, according to the [bolero instructions](https://camshaft.github.io/bolero/cli-installation.html)

**Debian/Ubuntu**:
```
$ sudo apt install binutils-dev libunwind-dev
```

**Nix**:
```
$ nix-shell -p libbfd libunwind libopcodes
```

## Project Setup

When setting up a project for analysis, you need to do the following in its `Cargo.toml`.

### Add the bolero dev dependency

```toml
[dev-dependencies]
bolero = { git = "https://github.com/soteria-tools/bolero", rev = "bdcfbb840fd1022220d150fb9ced192af182ad06" }
```

### Add the fuzz profile (required for fuzzing)

Following the [bolero instructions](https://camshaft.github.io/bolero/library-installation.html), you need to add the following to your `Cargo.toml`.

```toml
[profile.fuzz]
inherits = "dev"
opt-level = 3
incremental = false
codegen-units = 1
```

### Declare test targets

Soteria requires tests to be in **separate build targets** — it cannot find tests defined inside `src/lib.rs` or `src/main.rs`.

Create a `tests/` folder at the root of your project and add one file per harness. For example, `tests/test_feature1.rs`. Then declare the target in `Cargo.toml`:

```toml
[[test]]
name = "test_feature1"
path = "tests/test_feature1.rs"
```

For instance, in this repository, the test file is called `test.rs`, so we have the following in [Cargo.toml](./Cargo.toml):
```toml
[[test]]
name = "test"
path = "tests/test.rs"
```

## Writing Tests

A bolero harness uses `bolero::check!()` to generate arbitrary inputs and verify a property over all of them. In this repo, `tests/test.rs` contains:

```rust
use bolero_test::buggy_add;

#[test]
fn fuzz_add() {
    bolero::check!()
        .with_type()
        .cloned()
        .for_each(|(a, b)| buggy_add(a, b) == a.wrapping_add(b));
}
```

The target is declared in `Cargo.toml` as:

```toml
[[test]]
name = "test"
path = "tests/test.rs"
```

## Running Tests

### With Soteria (symbolic execution)

Pass `-e soteria` to run the harness under symbolic execution. Soteria explores all code paths exhaustively, finding the bug almost instantly:

```sh
cargo bolero test fuzz_add -e soteria
```

### With fuzzing (standard bolero)

Without the `-e soteria` flag, bolero runs as a standard fuzzer. It will eventually find the bug, but may take longer since it relies on random input generation:

```sh
cargo bolero test fuzz_add
```

For more details on using bolero, see the [bolero book](https://camshaft.github.io/bolero/introduction.html).

## Configuration

The following environment variables can be set to tune Soteria's behaviour:

| Variable                  | Description                                              | Example           |
|---------------------------|----------------------------------------------------------|-------------------|
| `SOTERIA_SOLVER_TIMEOUT`  | Solver timeout in milliseconds per query                 | `SOTERIA_SOLVER_TIMEOUT=10` |
| `STEP_FUEL`               | Maximum number of statements executed per branch         | `STEP_FUEL=10000` |
| `BRANCH_FUEL`             | Maximum number of branching points explored              | `BRANCH_FUEL=5`   |
