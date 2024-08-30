SHARE_DIR = $(HOME)/.local/share/nasin
LIB_DIR = $(SHARE_DIR)/library

.PHONY: all
all: bin/nasin $(LIB_DIR)

.PHONY: clean
clean:
	rm -rf bin
	rm -rf tree-sitter-nasin/node_modules
	rm -rf tree-sitter-nasin/src
	rm -rf $(LIB_DIR)
	cargo clean

.PHONY: test
test: bin/nasin $(LIB_DIR)
	./rere.py replay tests/_test.list

.PHONY: record-test
record-test: bin/nasin
	./rere.py record tests/_test.list

LIB_SRC = $(shell find library/ -type f -name '*.nsn')
$(LIB_DIR): $(LIB_SRC)
	mkdir -p $(basename $(LIB_DIR)) \
	&& rm -rf $(LIB_DIR)            \
	&& cp -r library $(LIB_DIR)

RUST_SRC = $(shell find src/ -type f -name '*.rs')
bin/nasin: Cargo.toml $(RUST_SRC) tree-sitter-nasin/src
	LIB_DIR=$(LIB_DIR) cargo build \
	&& mkdir -p bin                \
	&& cp -T target/debug/nasin bin/nasin

tree-sitter-nasin/src: tree-sitter-nasin/grammar.js tree-sitter-nasin/node_modules
	cd tree-sitter-nasin        \
	&& bun tree-sitter generate \
	&& bun tree-sitter build

tree-sitter-nasin/node_modules: tree-sitter-nasin/package.json
	cd tree-sitter-nasin \
	&& bun install
