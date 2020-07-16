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
fn test(a: Vec<&str>, b: Test, mut c: calcite::ArrayBuffer<i32>) -> i32 {
    println!("Got first argument {:?}", a);
    println!("Got second argument {:?}", b);
    println!("Got third argument {:?}", c.as_slice());
    c.as_mut_slice()[0] = 45;
    8
}

calcite::export!(test);
