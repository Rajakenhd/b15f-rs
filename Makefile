b15f:
	cargo build

test:
	cargo test

bench:
	cargo bench

doc:
	cargo doc
	rm -rf doc
	cp -r target/doc .

all: b15f test

.PHONY: all