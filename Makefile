.PHONY: help build run demo schemes test lint fmt outdated install check

.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo "  build     cargo build"
	@echo "  run       make run ARGS='schemes list'"
	@echo "  demo      how to use bin/demo.sh (mosmetro schema)"
	@echo "  schemes   list available transliteration schemas"
	@echo "  test      run tests"
	@echo "  lint      run clippy"
	@echo "  fmt       check formatting"
	@echo "  outdated  check for outdated dependencies (needs cargo-outdated)"
	@echo "  install   release build, copy to ~/.local/bin"
	@echo "  check     run fmt + lint + test"

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
