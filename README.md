# Inscriptions Substreams

> Substreams for Inscriptions.

## Quickstart

```
$ gh repo clone pinax-network/substreams-inscriptions
$ cd substreams-inscriptions
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/substreams-inscriptions/releases

## References

- [Avascriptions](https://docs.avascriptions.com/)
- [IERC 20](https://www.ierc20.com/)

### Mermaid Graph

```mermaid
graph TD;
  map_operations[map: map_operations];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_operations;
  graph_out[map: graph_out];
  map_operations --> graph_out;
```

### Modules

```yaml
Package name: inscriptions
Version: v0.1.0
Doc: Inscriptions
Modules:
----
Name: map_operations
Initial block: 0
Kind: map
Output Type: proto:inscriptions.types.v1.OperationsEvent
Hash: 0e76f8667de427579ecc0a062b9362f2da888d77
Doc:  Extracts 'Operations' events from the block

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 9b4c9974527d76debb932e5412679f3010079731
```
