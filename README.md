# Calcite
[![Crates.io](https://img.shields.io/crates/v/calcite)](https://crates.io/crates/calcite)

Calcite is a library for easily creating deno plugins.

# Getting Started
Refer [the docs](https://github.com/Srinivasa314/calcite/tree/master/docs)

# Examples
To run the [examples](https://github.com/Srinivasa314/calcite/tree/master/examples) do
```
cargo build --example sync
deno run --unstable --allow-plugin --allow-read --allow-write examples/sync.ts
cargo build --example async
deno run --unstable --allow-plugin --allow-read --allow-write examples/async.ts
```

# Example libraries created using calcite
* [tinyfiledialogs-deno](https://github.com/Srinivasa314/tinyfiledialogs-deno/): A port of tinyfiledialogs-rs for use in Deno
