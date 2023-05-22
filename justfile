alias t := test

clippy:
	cargo clippy --all --all-targets --features "all_codecs daemon spotify"

testall:
	cargo test --all --no-fail-fast --all-features

test:
	cargo test --all --no-fail-fast --features "all_codes daemon spotify"
