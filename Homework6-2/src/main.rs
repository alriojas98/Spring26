//Assigment 4
use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const TERMINATION_SIGNAL: i32 = -1;
const NUM_PRODUCERS: usize = 2;
const NUM_CONSUMERS: usize = 3;
const ITEM_COUNT: usize = 20;

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
    let remainder = ITEM_COUNT % NUM_PRODUCERS;

    let mut producer_handles = Vec::with_capacity(NUM_PRODUCERS);
    for id in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        let item_count = items_per_producer + usize::from(id < remainder);
        producer_handles.push(thread::spawn(move || producer(id + 1, tx, item_count)));
    }

    let mut consumer_handles = Vec::with_capacity(NUM_CONSUMERS);
    for id in 0..NUM_CONSUMERS {
        let rx = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || consumer(id + 1, rx)));
    }

    for handle in producer_handles {
        handle.join().unwrap();
    }

    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let value = rng.gen_range(1..=100);
        println!("Producer {id} produced {value}");
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    println!("Producer {id} finished producing {item_count} items");
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {id} received termination signal and is exiting");
            break;
        }

        println!("Consumer {id} processed {value}");
        thread::sleep(Duration::from_millis(75));
    }
}
