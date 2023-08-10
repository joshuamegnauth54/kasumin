alias t := test

binkasu:
	cargo build --bin kasumin --features "all_codecs spotify"

clippy:
	cargo clippy --all --all-targets --features "all_codecs spotify"

testall:
	cargo test --all --no-fail-fast --all-features

test:
	cargo test --all --no-fail-fast --features "all_codecs spotify"
