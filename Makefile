run: src/main.rs
	cargo run test


build: src/main.rs
	cargo build --release -q
	chmod a+x ./target/release/identicon
