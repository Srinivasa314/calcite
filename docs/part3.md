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
import { loadPlugin, importFromPlugin, importAsyncFromPlugin } from 'https://deno.land/x/calcite@2.1/calcite.ts';

```

Then call it from Deno!

```ts
const sleep = importAsyncFromPlugin("sleep") as (secs: number) => Promise<String>
sleep(3).then((response) => console.log(response))
```

NOTE:
Unfortunately ArrayBuffer and ReturnBuffer cannot be used in async functions
