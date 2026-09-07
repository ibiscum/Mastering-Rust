# Error-handling patterns in Chapter05

This chapter demonstrates the main error-handling tools available in Rust and
when each is appropriate.

| File | Pattern | Use case |
|---|---|---|
| `catch_unwind.rs` | `panic::catch_unwind` | Isolate a panic boundary, e.g. when a foreign callback or plugin must not crash the host process. |
| `panic.rs` | `panic!` | Programming errors and unrecoverable invariants. The example uses division by zero as a classic invariant violation. |
| `result-1.rs` | `match` on `Result` | The most explicit form of error handling; use when you need fine-grained control over each branch. |
| `result-unwrapping.rs` | `expect` | Prototypes and cases where failure means the environment is misconfigured and recovery is impossible. |
| `mapping.rs` | `Result::map` / combinators | Transform success values without unwrapping; keeps code flat and composable. |
| `qmark.rs` | `?` operator | The idiomatic way to propagate errors in functions that return `Result` or `Option`. |
| `try.rs` | `match` vs `?` comparison | Shows that `?` is syntactic sugar for early-return on `Err`. |
| `try-main.rs` | Propagating from helpers | Demonstrates returning `Result` from helper functions and handling it once at the top level. |
| `custom-error-1.rs` | Custom error type | Returning a domain-specific error instead of panicking. |
| `custom-error-2.rs` | Error chaining with `source` | Attaching an underlying cause so callers can inspect the full error chain. |

## Guidelines

- Prefer `Result` over `panic!` for recoverable failures.
- Use `?` to propagate errors; extract helper functions when indentation grows.
- Provide `Display` and, when wrapping another error, `Error::source`.
- Avoid `Error::description` and `Error::cause`; both are deprecated.
- Use `expect`/`unwrap` only when a failure indicates a bug or a precondition that the caller cannot fix.
