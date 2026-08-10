# Design: porting string.c to Rust
Draft, for review by Jakub Jelen / Andreas Schneider.
Repo: https://github.com/kantnero/libssh-string.rs

## Goal
Replace string.c with a Rust implementation that provides the same
`ssh_string_*` API surface, so the rest of libssh doesn't need to change to
build against it. Alongside that, add a CMake module to build/link the
Rust code, based on CMake 4.3's experimental Rust support. Stretch goal:
same treatment for buffer.c.
This writes up where things landed on ownership, zeroization, and struct
layout, and tracks known issues found in the current implementation, so
they're fixed before this is proposed upstream.
## Current C implementation (reference)
```c
struct ssh_string_struct {
    uint32_t size;
    unsigned char data[1]; /* flexible array member */
};
```
## Ownership
Allocation and freeing must not be split across the language boundary.
Rust owns the memory for the whole lifetime of a string:
- Allocation happens in Rust (`ssh_string_new` and friends).
- Freeing goes through one function, `ssh_string_free()` implemented in Rust.
- Every string function must call `ssh_string_free()`, never `free()`
  directly, mixing allocators across the boundary is undefined behavior
  in general.
Follow-on work: every place in libssh that currently does
`free(str)`/`SAFE_FREE(str)` on an `ssh_string` needs to be found and changed to
`ssh_string_free()`.
## Struct layout (implemented)
```rust
#[repr(C)]
struct SshStringStruct {
    data: *mut u8,
    size: usize,
}
```
This is the pointer + size design discussed earlier — two allocations
(struct header via `Box`, data buffer via `alloc_zeroed`/`Layout`). This matches
the recommended direction: it lets the implementation use ordinary Rust
allocation primitives instead of hand-rolling a flexible-array-member layout,
at the cost of requiring the C header to either change or become opaque, and
requiring the call-site audit (existing `->data`/`->size` access in libssh C
code) to land first. That header work isn't done yet — see open questions.
## ssh_burn / zeroization
Not implemented - A macro written in C to zero out memory the direction is to write a
rust equivalent inside `ssh_string_burn`.

## cbindgen vs. hand-written headers

Not resolved yet.

- **cbindgen**: auto-generates `string.h` from `string.rs`.
- **Hand-written**: a human controls exactly what's exposed, avoids a
  build-time cbindgen dependency for consumers, diffs cleanly against
  what's shipped today for upstream review.

## CMake

New module wraps the Rust build (cargo or equivalent) for the string.rs
crate as a static library and links it into the existing target, based on
CMake 4.3's experimental Rust support (see the
[CMake Rust integration plan](https://hackmd.io/@asn/r19DwWZwbx)). Not
implemented yet.

## Testing

- Rust unit tests: alloc/free, burn-then-read is all zero, round trip
  through `ssh_string_from_char`/`ssh_string_to_char`, empty-string edge
  cases, fill-then-read-full-size, copy producing an independent buffer
  with matching contents, cmp across equal/unequal size and content.
- Run the existing C tests for string.c against the new implementation
  for behavioral parity.
- A test asserting the struct's field layout/size, so a future change
  can't silently break the ABI.

## Call-site audit

Anything in libssh that calls `free()`/`SAFE_FREE()` on an `ssh_string`
directly, needs updating — a prerequisite for the opaque-struct header,
not follow-up cleanup.

## Documentation

Doxygen comments go in the C header, matching libssh's existing style —
`@brief`, `@param[in]`/`@param[out]`, `@returns`, `@see` between related
functions, `@warning` for anything security-relevant.

```c
/**
 * @brief Deallocate an SSH string object.
 *
 * @param[in]  str  The SSH string to free. May be NULL, in which case
 *                  this function is a no-op.
 *
 * @see ssh_string_burn()
 */
void ssh_string_free(ssh_string str);
```

## Open questions

1. Sign off on struct-layout direction as implemented.
2. cbindgen vs. hand-written header, or cbindgen as CI-only drift check.

Note: AI assistance (Claude) was used to help draft and organize the
wording of this document. The Rust implementation and code are entirely
the author's own work.
