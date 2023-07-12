alias t := test

binkasu:
	cargo build --bin kasumind --features "all_codecs daemon spotify"

clippy:
	cargo clippy --all --all-targets --features "all_codecs daemon spotify"

testall:
	cargo test --all --no-fail-fast --all-features

test:
	cargo test --all --no-fail-fast --features "all_codecs daemon spotify"
