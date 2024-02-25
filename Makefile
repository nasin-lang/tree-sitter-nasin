.PHONY: all
all: bin/torvo bin/torvo-parser

.PHONY: clean
clean:
	rm -rf bin
	rm -rf parser/node_modules
	rm -rf parser/src/proto
	cargo clean

.PHONY: proto
proto: proto/ast.proto proto/lex.proto parser/src/proto/ast.ts
	cargo clean && cargo build

RUST_SRC = $(shell find src/ -type f -name '*.rs')
bin/torvo: Cargo.toml build.rs $(RUST_SRC) proto/ast.proto proto/lex.proto
	cargo build     \
	&& mkdir -p bin \
	&& cp -T target/debug/torvo bin/torvo

PARSER_SRC = $(shell find parser/src/ -type f -name '*.ts')
bin/torvo-parser: $(PARSER_SRC) parser/src/proto/ast.ts parser/node_modules
	cd parser          \
	&& mkdir -p ../bin \
	&& bun build src/index.ts --outfile ../bin/torvo-parser --compile

parser/src/proto/ast.ts: proto/ast.proto parser/node_modules
	cd parser             \
	&& mkdir -p src/proto \
	&& protoc -I ../proto --plugin ./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out src/proto ../proto/ast.proto --ts_proto_opt=emitDefaultValues=json-methods

parser/node_modules: parser/package.json
	cd parser \
	&& bun install
