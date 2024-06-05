.PHONY: all
all: bin/torvo

.PHONY: clean
clean:
	rm -rf bin
	rm -rf parser/node_modules
	rm -rf parser/src/proto
	cargo clean

.PHONY: test
test: bin/torvo
	./rere.py replay tests/_test.list

.PHONY: record-test
record-test: bin/torvo
	./rere.py record tests/_test.list

RUST_SRC = $(shell find src/ -type f -name '*.rs')
bin/torvo: Cargo.toml $(RUST_SRC) tree-sitter-torvo/src/parser.c
	cargo build     \
	&& mkdir -p bin \
	&& cp -T target/debug/torvo bin/torvo

tree-sitter-torvo/src/parser.c: tree-sitter-torvo/grammar.js tree-sitter-torvo/node_modules
	cd tree-sitter-torvo \
	&& bun run generate

tree-sitter-torvo/node_modules: tree-sitter-torvo/package.json
	cd tree-sitter-torvo \
	&& bun install
