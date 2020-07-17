//CREATE A PROPER EXAMPLE
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Test<'a> {
    num: i32,
    name: &'a str,
}

#[calcite::deno_op]
fn test(a: Vec<&str>, b: Test, mut c: calcite::ArrayBuffer<i32>) -> i32 {
    println!("Got first argument {:?}", a);
    println!("Got second argument {:?}", b);
    println!("Got third argument {:?}", c.as_slice());
    c.as_mut_slice()[0] = 45;
    8
}

#[calcite::deno_op]
async fn async_test(msg: &str, secs: u64) -> Result<u64, String> {
    let (tx, rx) = futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(secs));
        tx.send(()).unwrap();
    });
    rx.await.unwrap();
    println!("Got message {}", msg);
    Err(format!("ERROR SLEPT FOR {} seconds", secs))
}

#[calcite::deno_op]
fn foo() -> Result<String, String> {
    Err("heya".into())
}

calcite::export!(test, async_test, foo);
