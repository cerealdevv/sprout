.PHONY: check fmt clippy test format
check: fmt clippy test
fmt:
	cargo fmt --check
clippy:
	cargo clippy --all-targets --all-features -- -D warnings
test:
	# cargo test # we dont have any unittests here, so ignore it.
format:
	cargo fmt
