PREFIX ?= $(HOME)/.local
DESTDIR ?=
SPROUT_SOURCE ?= make
BIN := target/release/sprout

export SPROUT_SOURCE

.SILENT:
.PHONY: check fmt clippy test format build install uninstall

check: fmt clippy test

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	echo "SPROUT has no unittests as of v0.3-wip"

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
