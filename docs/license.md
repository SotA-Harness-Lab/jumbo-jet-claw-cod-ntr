## License

This repository is licensed under the [Apache-2.0 License](../LICENSE).

The upstream project's license text is kept in [LICENSES/Codex-Apache-2.0.txt](../LICENSES/Codex-Apache-2.0.txt), and
the README's License section lists the third-party code in this repository.

## Third-party code built into the released programs

Some third-party source in this repository is compiled into a program that is released, without being a Cargo
dependency, so the tools that collect dependency licenses from `Cargo.lock` do not see it. A release has to handle
each of these by hand.

| Code | Where | License | Built into | A release must |
|------|-------|---------|------------|----------------|
| bubblewrap 0.11.2 | `codex-rs/vendor/bubblewrap` | LGPL-2.0-or-later | `bwrap`, the Linux sandbox helper (`codex-rs/bwrap/build.rs` compiles it with `cc`) | ship `codex-rs/vendor/bubblewrap/COPYING` with the Linux packages, and say that the source is in that directory of this repository at the release's tag and that `bwrap` can be rebuilt from a changed copy with `cargo build -p codex-bwrap` |

When code is vendored, updated or removed, or a build script starts compiling another vendored directory, update this
table in the same change.
