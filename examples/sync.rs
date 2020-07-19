#[macro_use]
extern crate serde_derive;

#[calcite::deno_op]
fn array_buffer_example(mut arr: calcite::ArrayBuffer<i32>, num: i32) {
    for (pos, val) in arr.as_mut_slice().iter_mut().enumerate() {
        *val = num + pos as i32;
    }
}

#[derive(Deserialize, Debug)]
struct Struct1<'a> {
    a: i32,
    b: &'a str,
}

#[calcite::deno_op]
fn struct_example(s1: Struct1) {
    println!("Got argument {:?}", s1);
}

#[calcite::deno_op]
fn return_example(a: (i32, &str)) -> String {
    format!("I got {:?} as argument", a)
}

#[calcite::deno_op]
fn multiple_arguments_example(x: f32, y: (&str, &str)) {
    println!("Got arguments x:{} y:{:?}", x, y);
}

#[calcite::deno_op]
fn return_buffer() -> calcite::ReturnBuffer {
    calcite::ReturnBuffer::from_bytes(vec![5, 7, 1, 3])
}

calcite::export!(
    array_buffer_example,
    struct_example,
    return_example,
    multiple_arguments_example,
    return_buffer
);
