import { loadPlugin, importFromPlugin, importAsyncFromPlugin } from 'file:///home/srikanth/rust/calcite-ts/calcite.ts';
///CHANGE TO DENO>LAND OR DENOPKG!!!!

await loadPlugin("hello_world", "file://target/debug/examples/")

const test = importFromPlugin("test");

let arr = new Int32Array([1, 4, 7]);
console.log(`Got result ${test(["hello", "hi"], { num: 8, name: "abc" }, arr)} from plugin`)

console.log(`Value of arr after function call is ${arr}`)

const async_test = importAsyncFromPlugin("async_test");

async_test("hey sleep for 5 seconds", 5).then((reply) => console.log(`Got ${reply}`)).catch((err)=>console.log(`Got err ${err}`));
async_test("hey sleep for 3 seconds", 3).then((reply) => console.log(`Got ${reply}`)).catch((err)=>console.log(`Got err ${err}`));
