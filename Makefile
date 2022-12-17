b15f:
	cargo build

test:
	cargo test

bench:
	cargo bench

doc:
	rm -rf doc
	rm -rf target/doc
	cargo doc --no-deps
	cp -r target/doc .

all: b15f test

.PHONY: all