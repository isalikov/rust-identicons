run: src/main.rs
	cargo run


build: src/main.rs
	cargo build --release -q
	chmod a+x ./target/release/identicons
