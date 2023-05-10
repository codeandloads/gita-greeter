DESTDIR = ~/.local
HOMEDIR = $(HOME)
PREFIX = $(DESTDIR)/bin

build:
	cargo build --release --locked

install:
	mkdir -p ~/.local/bin
	cp ./target/release/gita_greeter $(PREFIX)
	cp bhagavad.sqlite $(HOMEDIR)

uninstall:
	rm $(PREFIX)/gita_greeter
	rm $(HOMEDIR)/bhagavad.sqlite

package:
	tar czf gita-greeter.tar.gz bhagavad.sqlite -C target/release/ gita_greeter
