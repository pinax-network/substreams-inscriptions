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

.PHONY: gui
gui:
	substreams gui db_out -e avalanche.substreams.pinax.network:443 -s 38209552

.PHONY: setup
setup:
	substreams-sink-sql setup clickhouse://default:@localhost:9000/default substreams.yaml

.PHONY: run
run:
	substreams-sink-sql run clickhouse://default:@localhost:9000/default substreams.yaml -e avalanche.substreams.pinax.network:443 38209552: --final-blocks-only --undo-buffer-size 100 --flush-interval 10000

