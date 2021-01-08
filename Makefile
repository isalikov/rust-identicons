build: src/main.rs
	git clone https://github.com/isalikov/rust-identicons.git ~/.identicons
	cargo build --release -q
	chmod a+x ./target/release/identicon

install: target/release/identicon
	cp ./target/release/identicon /usr/local/bin
	rm -rf ~/.identicons
