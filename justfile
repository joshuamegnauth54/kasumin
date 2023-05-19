alias t := test

testall:
	cargo test --all --no-fail-fast --all-features

test:
	cargo test --all --no-fail-fast --features daemon
