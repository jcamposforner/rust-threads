use std::sync::{Mutex, mpsc, Arc};
use std::thread;

// DIVIDE EL NUMERO PASADO ENTRE 2 y N
fn is_prime(n: usize) -> bool {
    (2..n).all(|i| { n % i != 0 })
}

fn producer(tx: mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    thread::spawn(move || for i in 100_000_000.. {
        tx.send(i).unwrap();
    })
}

fn worker(id: u64, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    thread::spawn(move || loop {
        {
            let mut n = 0;
            match shared_rx.lock() {
                Ok(rx) => {
                    match rx.try_recv() {
                        Ok(_n) => {
                            n = _n;
                        },
                        Err(_) => ()
                    }

                    if n != 0 {
                        if is_prime(n) {
                            println!("worker {} found prime {}",id, n);
                        }
                    }
                },
                Err(_) => ()
            }
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1024);
    let shared_rx = Arc::new(Mutex::new(rx));

    for i in 1..5 {
        worker(i, shared_rx.clone());
    }

    producer(tx).join().unwrap();
}
