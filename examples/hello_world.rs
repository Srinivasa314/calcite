//CREATE A PROPER EXAMPLE
use calcite::deno_op;
use deno_core::ZeroCopyBuf;

#[deno_op]
pub fn a(args: &[ZeroCopyBuf]) -> Box<[u8]> {
    for arg in args {
        println!("Function a got arg {:?}", &arg[..])
    }
    Box::new([1, 2, 3])
}

#[deno_op]
fn b(args: &[ZeroCopyBuf]) -> Box<[u8]> {
    for arg in args {
        println!("Function b got arg {:?}", &arg[..])
    }
    Box::new([9, 8])
}

#[deno_op]
async fn c(args: &[ZeroCopyBuf]) -> Box<[u8]> {
    let (tx, rx) = futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
        tx.send(()).unwrap();
    });
    rx.await.unwrap();
    for arg in args {
        println!("Function c got arg {:?}", &arg[..])
    }
    Box::new([9, 8, 7])
}

calcite::export!(a, b, c);
