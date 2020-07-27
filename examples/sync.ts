import {
  loadPlugin,
  importFromPlugin,
} from "https://deno.land/x/calcite@2.0/calcite.ts";
await loadPlugin("sync", "file://target/debug/examples/");

const array_buffer_example = importFromPlugin("array_buffer_example") as (
  arr: Int32Array,
  num: number,
) => void;
const struct_example = importFromPlugin("struct_example") as (
  s1: { a: number; b: string },
) => void;
const return_example = importFromPlugin("return_example") as (
  a: [number, string],
) => string;
const multiple_arguments_example = importFromPlugin(
  "multiple_arguments_example",
) as (x: number, y: [string, string]) => void;
const return_buffer = importFromPlugin(
  "return_buffer",
  { returnRawBuffer: true },
) as () => Uint8Array;

let a = new Int32Array(5);
array_buffer_example(a, 8);
console.log(`The value of a is ${a}`);

struct_example({ a: 75, b: "Hey!" });
console.log(
  `The result of return_example is "${return_example([98, "Bye!"])}"`,
);

multiple_arguments_example(8.9, ["abc", "xyz"]);
console.log(return_buffer());
