build: src/main.rs
	cargo build --release -q
	chmod a+x ./target/release/identicon

install: target/release/identicon
	cp ./target/release/identicon /usr/local/bin
