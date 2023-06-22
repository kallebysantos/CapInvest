use std::{
    sync::mpsc::{channel, TryRecvError},
    thread,
    time::Duration,
};

fn main() {
    let orders_in = channel::<u8>();
    let orders_out = channel::<u8>();

    thread::Builder::new()
        .name("kafka-listener".into())
        .spawn(move || loop {
            // List Kafka here
            // Send order to orders_in
            let _ = orders_in.0.send(5).unwrap();
            println!("Sent 5 from kafka-listener");

            thread::sleep(Duration::from_secs(5))
        })
        .unwrap();

    thread::Builder::new()
        .name("trade-matcher".into())
        .spawn(move || loop {
            // Receive from orders_in
            // Send order to orders_out
            if let Ok(value) = orders_in.1.try_recv() {
                println!("Receive {} inside trade-matcher", value);

                let _ = orders_out.0.send(value * 2);
            }
        })
        .unwrap();

    // Order publisher
    loop {
        match orders_out.1.try_recv() {
            Ok(value) => println!("Receive {} in main", value), // Publish processed order Kafka here
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => {
                panic!("The channel has been disconnected, shutting down.");
            }
        }
    }
}
