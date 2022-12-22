# adr-tools

This is a Rust-based set of tools for managing Architectural Decision Records
(ADRs).

We use embedded [SurrealDB](https://surrealdb.com/).

Markdown files are loaded into the graph database, which allows us to do all
sorts of cool analytics.

The CLI is effectively a port of
[`adr-tools`](https://github.com/npryce/adr-tools).

## Contributing

### Nightly Rust

```shell
rustup default nightly
```