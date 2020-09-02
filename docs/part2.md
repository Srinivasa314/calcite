The exported functions can take any type and return any type
For example let us create a function that checks whether a person is an adult. 
Values passed to functions and returned from functions exported by calcite are stored as JSON. To serialize and deserialize structs add serde_derive and serde to dependencies.Let us declare a struct Person
```rust
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Person<'a> {
    name: &'a str,
    age: u32
}
```
Then we can declare the function
```rust
#[calcite::deno_op]
fn is_adult (p: Person, age_limit: u32) -> bool {
    if p.age > age_limit {
        true
    }
    else {
        false
    }
}
```
We should then add it to the exported function list
```rust
calcite::export!(multiply, is_adult);
```

Now let us call it from Deno
```ts
const is_adult = importFromPlugin('is_adult') as (p: {name: string, age: number}, age_limit: number) => boolean
console.log(is_adult({name:'abc', age:16}, 18))
console.log(is_adult({name:'xyz', age:25}, 23))
```

Now we can build it and run it as before

NOTE:
Since the plugin is cached in .deno_plugins the new built plugin will not be used so delete the cached plugin

Calcite uses serde_json for serializing and deserializing so any type that implements Serialize and Deserialize traits of serde can be used. 

## ArrayBuffer
To modify javascript/typescript arrays you must use Int32Array, Float64Array ,etc.
Let us create a function to sort an array of numbers.
Calcite provides the ArrayBuffer type to do this. So just simply replace `&[u8]`, `&[f32]` in rust functions with the ArrayBuffer type.
```rust
#[calcite::deno_op]
fn sort (mut nums: calcite::ArrayBuffer<u32>) {
    nums.as_mut_slice().sort()
}
```
Export the function as before.

In the typescript file add
```ts
const sort = importFromPlugin("sort") as (nums: Uint32Array) => void;
let a = new Uint32Array([9, 3, 7, 10]);
sort(a);
console.log(a);
```

Now we can run it as before.

## ReturnBuffer
Values returned from functions exported by calcite are serialized using JSON. Say you want to return a `Vec<u8>` but do not want this serialization to happen. Then you can use `calcite::ReturnBuffer`

For example,

```rust
use calcite::ReturnBuffer;

#[calcite::deno_op]
fn return_buff() -> ReturnBuffer {
    ReturnBuffer::from_bytes("Hey".to_string().into_bytes())
}
 
```

The returnRawBuffer option must be set to true while importing the function
```ts
const return_buff = importFromPlugin("return_buff", {returnRawBuffer: true}) as () => Uint8Array
console.log(return_buff())
```
