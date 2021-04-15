use async_std::task::{block_on, spawn};
use async_std::sync::{Arc, Mutex};
use futures::future::join;

type ArcMutexI32 = Arc<Mutex<i32>>;

async fn work(value: ArcMutexI32) {
    let mut m_value = value.lock().await;
    for _ in 0..1000 {
        *m_value += 1;
    }
}

fn main() {
    println!("Hello, world!");
    
    let value = Arc::new(Mutex::new(0i32));
    
    let work_a = spawn(work(value.clone()));
    let work_b = spawn(work(value.clone()));

    block_on(async {
        join(work_a, work_b).await;

        let m_value = value.lock().await;
        println!("result : {}", m_value);
    });
}
