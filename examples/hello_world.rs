//CREATE A PROPER EXAMPLE
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Test<'a> {
    num: i32,
    name: &'a str,
}
//use deno_core::ZeroCopyBuf;
#[calcite::deno_op]
fn test(a: Vec<&str>, b: Test) -> i32 {
    println!("Got first argument {:?}", a);
    println!("Got second argument {:?}", b);
    8
}

calcite::export!(test);
