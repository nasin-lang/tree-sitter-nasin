torvo: torvo-parser

torvo-parser: node_modules src/proto/ast.ts src/proto/m_ir.ts
	bun build src/index.ts --outfile bin/torvo-parser --compile

src/proto/m_ir.ts: node_modules
	protoc -I proto --plugin ./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out src/proto proto/m_ir.proto --ts_proto_opt=emitDefaultValues=json-methods

src/proto/ast.ts: node_modules
	protoc -I proto --plugin ./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out src/proto proto/ast.proto --ts_proto_opt=emitDefaultValues=json-methods

node_modules:
	bun install

