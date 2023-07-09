use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, TryRecvError},
        Arc,
    },
    thread,
};

use trade_wara::{entities::order::OrderItem, order_book::OrderBook};

fn main() {
    let mut book_hash = HashMap::new();
    let orders_in = channel::<Arc<dyn OrderItem>>();
    let orders_out = channel::<Arc<dyn OrderItem>>();

    thread::Builder::new()
        .name("kafka-listener".into())
        .spawn(move || loop {
            // List Kafka here

            // Parse input orders from Kafka

            // Send order to orders_in
        })
        .unwrap();

    thread::Builder::new()
        .name("trade-matcher".into())
        .spawn(move || loop {
            if let Ok(order) = orders_in.1.try_recv() {
                let book = book_hash
                    .entry(order.asset_id())
                    .or_insert(OrderBook::new(order.asset_id()));

                match book.append(order.resolve_type()) {
                    Err(err) => panic!("{:?}", err),
                    Ok(()) => {
                        // Do order match here

                        // Send order to orders_out
                    }
                }
            }
        })
        .unwrap();

    // Order publisher
    loop {
        match orders_out.1.try_recv() {
            Ok(_) => println!("Receive in main"), // Publish processed order Kafka here
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => {
                panic!("The channel has been disconnected, shutting down.");
            }
        }
    }
}
