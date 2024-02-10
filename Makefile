.ONESHELL:
.PHONY: all clean

all: bin/torvo bin/torvo-parser

clean:
	rm -rf bin
	rm -rf parser/node_modules
	rm -rf parser/src/proto
	cargo clean

bin/torvo: Cargo.toml build.rs src/* proto/ast.proto proto/lex.proto
	cargo build
	cp -T target/debug/torvo bin/torvo

bin/torvo-parser: parser/src/* parser/src/proto/ast.ts parser/node_modules
	cd parser
	bun build src/index.ts --outfile ../bin/torvo-parser --compile

parser/src/proto/ast.ts: proto/ast.proto parser/node_modules
	cd parser
	mkdir -p src/proto
	protoc -I ../proto --plugin ./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out src/proto ../proto/ast.proto --ts_proto_opt=emitDefaultValues=json-methods

parser/node_modules: parser/package.json
	cd parser
	bun install
