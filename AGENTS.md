# AGENTS.md

## Project
CLI tool for Cyrillic transliteration, wrapping `iuliia-rust`.
Stack: Rust (edition 2024, MSRV 1.85), no database.

## Commands
`make check` runs fmt, clippy, and tests. `make help` lists individual targets.

## Definition of Done
`make check` is green and the feature's specific verification passes.

## Work rules
Priority: correctness → performance → style.

## Escalate to human (stop and ask)
- Changing the minimum supported Rust version.
- Adding a new runtime dependency.
- Changing the public CLI interface (tests in `tests/cli.rs` protect it).
- Deleting anything that cannot be recovered.
