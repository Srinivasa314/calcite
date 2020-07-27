import {
  loadPlugin,
  importAsyncFromPlugin,
} from "https://deno.land/x/calcite@2.1/calcite.ts";
await loadPlugin("async", "file://target/debug/examples/");

const sleep_for = importAsyncFromPlugin("sleep_for") as (
  secs: number,
) => Promise<string>;

sleep_for(5).then((response) => console.log(response));
sleep_for(3).then((response) => console.log(response));
sleep_for(-1).catch((err) => console.log(`Got error ${err}`));
