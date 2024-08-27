.PHONY: all
all: bin/nasin

.PHONY: clean
clean:
	rm -rf bin
	rm -rf parser/node_modules
	rm -rf parser/src/proto
	cargo clean

.PHONY: test
test: bin/nasin
	./rere.py replay tests/_test.list

.PHONY: record-test
record-test: bin/nasin
	./rere.py record tests/_test.list

RUST_SRC = $(shell find src/ -type f -name '*.rs')
bin/nasin: Cargo.toml $(RUST_SRC) tree-sitter-nasin/src/parser.c
	cargo build     \
	&& mkdir -p bin \
	&& cp -T target/debug/nasin bin/nasin

tree-sitter-nasin/src/parser.c: tree-sitter-nasin/grammar.js tree-sitter-nasin/node_modules
	cd tree-sitter-nasin \
	&& bun run generate

tree-sitter-nasin/node_modules: tree-sitter-nasin/package.json
	cd tree-sitter-nasin \
	&& bun install
