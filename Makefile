.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run map_operations -e avalanche-mainnet.streamingfast.io:443 -s -1000
#19336559
.PHONY: gui
gui:
	substreams gui graph_out -e avalanche-mainnet.streamingfast.io:443 -s -1000

