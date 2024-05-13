.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: test
test:
	cargo test

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

.PHONY: gui
gui:
	substreams gui map_operations -e avalanche.substreams.pinax.network:443 -s 38927848

.PHONY: gui-eos
gui-eos:
	substreams gui map_operations -e eosevm.substreams.pinax.network:443 -s 23071495

.PHONY: setup
setup:
	substreams-sink-sql setup clickhouse://default:@localhost:9000/default substreams.yaml

.PHONY: run
run:
	substreams-sink-sql run clickhouse://default:@localhost:9000/default substreams.yaml -e avalanche.substreams.pinax.network:443 38209552: --final-blocks-only --undo-buffer-size 100 --flush-interval 10000 --on-module-hash-mistmatch warn

