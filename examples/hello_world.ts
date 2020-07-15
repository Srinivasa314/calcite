import { loadPlugin, importFromPlugin } from 'https://denopkg.com/Srinivasa314/calcite-ts/calcite.ts'

await loadPlugin("hello_world", "file://target/debug/examples/")

const test = importFromPlugin("test");

console.log(`Got result ${test(["hello", "hi"], { num: 8, name: "abc" })} from plugin`)