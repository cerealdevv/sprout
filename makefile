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

commit:
	echo "this feature is only for NON RELEASE commits."
	echo "this feature depends on cargo-edit, please make sure it is installed."
	echo "press enter to confirm that this is NOT a release and you have cargo-edit installed."
	read
	git add .
	read -rp "please enter a commit name: " cmt
	cargo set-version --bump patch
	git commit -m "$cmt"
