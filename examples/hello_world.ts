//CREATE A PROPER EXAMPLE
const filenameBase = "hello_world";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "windows") {
    filenameSuffix = ".dll";
    filenamePrefix = "";
}
if (Deno.build.os === "darwin") {
    filenameSuffix = ".dylib";
}

const rid = Deno.openPlugin(`target/debug/examples/${filenamePrefix}${filenameBase}${filenameSuffix}`)

// @ts-ignore
const { a, b, c } = Deno.core.ops();

// @ts-ignore
const response1 = Deno.core.dispatch(a, new Uint8Array([3, 4, 5]), new Uint8Array([1, 2]));
// @ts-ignore
const response2 = Deno.core.dispatch(b, new Uint8Array([2, 3, 4]));
// @ts-ignore
const response3 = Deno.core.dispatch(c, new Uint8Array([1, 8, 7]), new Uint8Array([0]));

console.log(`Responses from plugin ${response1} ${response2}`);

// @ts-ignore
Deno.core.setAsyncHandler(c, (response) => {
    console.log(response)
})

