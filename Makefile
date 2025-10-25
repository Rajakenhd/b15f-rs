b15f:
	cargo build

test:
	cargo test

bench:
	cargo bench

doc:
	rm -rf docs
	rm -rf target/doc
	cargo doc --no-deps
	mkdir -p docs
	cp -r target/doc/* docs
	echo "Adding index.html"
	@echo "<!DOCTYPE html>" > docs/index.html
	@echo "<meta http-equiv='refresh' content='0; URL=b15f/index.html'>" >> docs/index.html
	@echo "<link rel='canonical' href='b15f/index.html'>" >> docs/index.html

all: b15f test

.PHONY: all
