Calcite allows you to use rust functions in Deno almost effortlessly. In this guide we will create a basic Deno plugin.

First create a new project
```
 cargo new my-deno-plugin --lib
 cd my-deno-plugin
```

Make sure you have the __latest__ version of Deno

Then add calcite and deno_core as a dependency in Cargo.toml

Any function can be used in deno if `#[calcite::deno_op]` is put above it.
For example let us create a function to multiply two numbers
```rust
#[calcite::deno_op]
fn multiply (a: f64, b: f64) -> f64 {
    a*b
}
```

That function should be exported
```rust
calcite::export!(multiply);
```
Thats it!
Now lets build it. The plugin must be a dynamic library so add this to Cargo.toml.
```toml
[lib]
crate-type = ["cdylib"]
``` 
To build it do 
```
cargo build
```

Then create a file called my-deno-plugin.ts and first import calcite-ts 
```ts
import { loadPlugin, importFromPlugin } from 'https://deno.land/x/calcite@2.0/calcite.ts';
```

Then load the plugin. The plugin is usually loaded from a website where the binaries are located.They are then cached based on the url. For now we will import it from our local filesystem. The plugin is located at target/debug
```ts
await loadPlugin("my_deno_plugin", "file://target/debug")
```

Then import the multiply function
```ts
const multiply = importFromPlugin('multiply') as (a: number, b: number) => number
```

Then you can use the multiply function
```ts
console.log(multiply(1.1, 2.2))
```

Then we can run using
```
deno run --unstable --allow-plugin --allow-read --allow-write my-deno-plugin.ts
```
