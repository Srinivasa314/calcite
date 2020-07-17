#[calcite::deno_op]
async fn sleep_for(secs: i32) -> Result<String, &'static str> {
    if secs < 0 {
        Err("Number of seconds cannot be negative")
    } else {
        let (tx, rx) = futures::channel::oneshot::channel();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(secs as u64));
            tx.send(()).unwrap();
        });
        rx.await.unwrap();
        Ok(format!("Slept for {} seconds", secs))
    }
}

calcite::export!(sleep_for);
