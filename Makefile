.PHONY: help help-all build run demo schemes test lint fmt outdated install check

.DEFAULT_GOAL := help

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "  check    - Run fmt + lint + test"
	@echo "  build    - Cargo build"
	@echo "  run      - Run the binary (pass args via ARGS=...)"
	@echo "  test     - Run tests"
	@echo "  install  - Release build, copy to ~/.local/bin"
	@echo ""
	@echo "Run 'make help-all' for the full list, including internal targets."

help-all:
	@echo "Public targets:"
	@echo "  check    - Run fmt + lint + test"
	@echo "  build    - Cargo build"
	@echo "  run      - Run the binary (pass args via ARGS=...)"
	@echo "  test     - Run tests"
	@echo "  install  - Release build, copy to ~/.local/bin"
	@echo ""
	@echo "Internal targets (typically called via check):"
	@echo "  lint     - Run clippy"
	@echo "  fmt      - Check formatting"
	@echo "  schemes  - List available transliteration schemas"
	@echo "  demo     - How to use bin/demo.sh (mosmetro schema)"
	@echo "  outdated - Check for outdated dependencies (needs cargo-outdated)"

target/debug/translit:
	cargo build

build: target/debug/translit

run: target/debug/translit
	./target/debug/translit $(or $(ARGS),--help)

demo:
	@echo "Demo uses mosmetro schema. Run the script directly, not through make:"
	@echo "  ./bin/demo.sh <token-or-phrase>"

schemes: target/debug/translit
	./target/debug/translit schemes list

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check

outdated:
	cargo outdated -R

install:
	cargo build --release
	mkdir -p "$${HOME}/.local/bin"
	rm -f "$${HOME}/.local/bin/translit"
	cp "$$(pwd)/target/release/translit" "$${HOME}/.local/bin/translit"
	@echo "Installed to ~/.local/bin"

check: fmt lint test
	@echo "ALL CHECKS PASSED"
