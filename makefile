PREFIX ?= $(HOME)/.local
DESTDIR ?=
SPROUT_SOURCE ?= make
BIN := target/release/sprout

export SPROUT_SOURCE

.SILENT:
.PHONY: check fmt clippy test format build install uninstall commit

check: fmt clippy test

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

format:
	cargo fmt

build:
	cargo build --release

install: build
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 $(BIN) $(DESTDIR)$(PREFIX)/bin/
	echo "SPROUT installed to $(PREFIX)/bin. Make sure it is in your PATH."

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/sprout
	
.ONESHELL:
commit:
	echo "This feature is only for NON RELEASE commits."
	echo "This feature depends on cargo-edit. Please make sure it is installed."
	read -r -p "Press enter to confirm this is NOT a release and cargo-edit is installed. "
	cargo set-version --bump patch
	git add .
	read -r -p "Please enter a commit name: " cmt
	if [ -z "$$cmt" ]; then
		echo "Commit message cannot be empty."
		exit 1
	fi
	git commit -m "$$cmt"
