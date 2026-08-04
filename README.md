# libssh-rust

Introducing Rust into [libssh](https://www.libssh.org/), starting with a port
of `string.c` to Rust and a CMake module for Rust integration.

This project was proposed for GSoC 2026 but wasn't selected. After
discussing it with Jakub Jelen, I'm pursuing it independently as a
learning and contribution project, with the goal of eventually proposing
the CMake module upstream to libssh.

## Goals

- Port `string.c` → `string.rs`, preserving the existing C ABI so no or minimal
  changes are required elsewhere in libssh
- Create a CMake module for building and linking Rust code into libssh,
  based on CMake 4.3's experimental Rust support
- Stretch goal: port `buffer.c` → `buffer.rs`

## Status

- Early stage — design doc in progress.

## References

- [CMake Rust Integration Plan](https://hackmd.io/@asn/r19DwWZwbx)
- Possible mentors (from the original GSoC proposal): Jakub Jelen,
  Andreas Schneider

## License

matching libssh's license.

