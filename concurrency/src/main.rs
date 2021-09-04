use std::{sync::{Arc, Mutex, mpsc}, thread, time::Duration};

fn main() {
    // thread_spawn();
    // move_closure();
    // channel_comm();
    // single_threaded_mutex();
    // multi_threaded_mutex();
    deadlock();
}

fn thread_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap()
}

fn move_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("v = {:?}", v);
    });

    handle.join().unwrap();
}

fn channel_comm() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn single_threaded_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6; 
    }

    println!("m = {:?}", m);
}

fn multi_threaded_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn deadlock() {
    let a = Arc::new(Mutex::new(2));
    let b = Arc::new(Mutex::new(3));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = thread::spawn(move || {
        let mut a = a1.lock().unwrap();
        println!("h1 has acquired the lock for a");
        thread::sleep(Duration::from_secs(1));
        println!("h1 is waiting for the lock for b");
        let b = b1.lock().unwrap();
        println!("h1 has acquired the lock for b");
        *a += *b;
        println!("h1 is now releasing the lock for a and b");
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = thread::spawn(move || {
        let mut b = b2.lock().unwrap();
        println!("h2 has acquired the lock for b");
        thread::sleep(Duration::from_secs(1));
        println!("h2 is waiting for the lock for a");
        let a = a2.lock().unwrap();
        println!("h2 has acquired the lock for a");
        *b += *a;
        println!("h2 is now releasing the lock for a and b");
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("a = {}, b = {}", a.lock().unwrap(), b.lock().unwrap());
}