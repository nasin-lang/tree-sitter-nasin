torvo: bin/torvo-parser src/* proto/ast.proto proto/m_ir.proto
	@mkdir -p parser/src/proto
	cargo build && cp -T target/debug/torvo bin/torvo

bin/torvo-parser: parser/src/* proto/ast.proto
	@mkdir -p parser/src/proto
	cd parser && \
		bun install && \
		mkdir -p src/proto && \
		protoc -I ../proto --plugin ./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out src/proto ../proto/ast.proto --ts_proto_opt=emitDefaultValues=json-methods && \
		bun build src/index.ts --outfile ../bin/torvo-parser --compile
