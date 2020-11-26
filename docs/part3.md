## Async Functions
Before reading this knowledge of async rust functions is needed.
Async functions in javascript can either give a value or be rejected for some reason. So async functions exported to deno must return a `Result<T,E>`.
Let us create an async function called sleep
```rust
#[calcite::deno_op]
async fn sleep(secs: u64) -> Result<String,()> {
    let (tx, rx) = futures::channel::oneshot::channel();    //Create a channel
    std::thread::spawn(move || {    //Spawn a thread
        std::thread::sleep(std::time::Duration::from_secs(secs));   // Sleep
        tx.send(()).unwrap();   // Send () to channel after sleeping
    });
    rx.await.unwrap();  // await message from channel
    Ok(format!("Slept for {} seconds", secs))
}
```
As usual export it. Add `futures` to dependencies as we are using it.

We need to import it using importAsyncFromPlugin 
```ts
import { loadPlugin, importFromPlugin, importAsyncFromPlugin } from 'https://deno.land/x/calcite/calcite.ts';

```

Then call it from Deno!

```ts
const sleep = importAsyncFromPlugin("sleep") as (secs: number) => Promise<String>
sleep(3).then((response) => console.log(response))
```
If an error is returned from an async function exported from calcite then you can retreive it in javascript/typescript using the catch method of `Promise`. You can return an error of any type. If you are using async..await syntax then an exception is thrown when an error is returned, so you can use try..catch. 

NOTE:
Unfortunately ArrayBuffer and ReturnBuffer cannot be used in async functions

## Using dynamically typed variables
Dynamically typed variables can be passed to and returned from functions exported using calcite. You can do this by using the `Value` type from the `serde_json` crate in the function arguments or as the return type. `Value` has many useful methods and you can view its documentation [here](https://docs.serde.rs/serde_json/enum.Value.html).
