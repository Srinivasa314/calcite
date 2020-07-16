import { loadPlugin, importFromPlugin } from 'https://denopkg.com/Srinivasa314/calcite-ts/calcite.ts'

await loadPlugin("hello_world", "file://target/debug/examples/")

const test = importFromPlugin("test");

let arr = new Int32Array([1, 4, 7]);
console.log(`Got result ${test(["hello", "hi"], { num: 8, name: "abc" }, arr)} from plugin`)

console.log(`Value of arr after function call is ${arr}`)