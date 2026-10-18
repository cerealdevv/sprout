PREFIX ?= $(HOME)/.local
.SILENT:
.PHONY: check fmt clippy test format install
check: fmt clippy test
fmt:
	cargo fmt --check
clippy:
	cargo clippy --all-targets --all-features -- -D warnings
test:
	echo "SPROUT has no unittests as of v0.3-wip"
format:
	cargo fmt
install:
	SPROUT_SOURCE=make cargo build --release
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 target/release/sprout $(DESTDIR)$(PREFIX)/bin/
	echo "SPROUT installed to $(PREFIX)/bin. Make sure it is in your PATH."
