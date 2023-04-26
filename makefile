build:
	cargo build --release --locked

package:
	tar czf gita-greeter.tar.gz bhagavad.sqlite -C target/release/ gita_greeter 
