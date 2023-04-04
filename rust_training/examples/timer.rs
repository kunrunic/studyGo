use std::time::Duration;

use tokio::sync::mpsc::Sender;

pub struct Closer {
    _tx: Sender<bool>,
}

pub fn schedule(
    delay: Duration,
    interval: Duration,
    mut f: impl FnMut() + 'static + Send,
) -> Closer {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<bool>(1);

    tokio::spawn(async move {
        if !delay.is_zero() {
            tokio::select! {
                _ = tokio::time::sleep(delay) => {},
                _ = rx.recv() => { return; }
            }
        }

        loop {
            f();
            tokio::select! {
                _ = tokio::time::sleep(interval) => {},
                _ = rx.recv() => { return; }
            };
        }
    });

    Closer { _tx: tx }
}

fn main() -> Result<(), String> {
    let t = "test";
    let chnl = schedule(Duration::default(), Duration::from_secs(60), move || {
        println!("loop interval write = {}", t)
    });

    // chnl을 더이상 이용하지 않을때 호출 방법
    drop(chnl);

    Ok(())
}
