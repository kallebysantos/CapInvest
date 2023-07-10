use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, TryRecvError},
        Arc, Mutex,
    },
    thread,
};

use trade_wara::{
    entities::{order::OrderItem, transaction::Transaction},
    order_book::OrderBook,
};

fn main() {
    let book_hash = Arc::new(Mutex::new(HashMap::new()));

    let orders = channel::<Arc<dyn OrderItem>>();
    let transactions = channel::<Arc<Transaction>>();

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
            if let Ok(order) = orders.1.try_recv() {
                let mut book_hash = book_hash.lock().unwrap();

                let book = book_hash
                    .entry(order.asset_id())
                    .or_insert(OrderBook::new(order.asset_id()));

                let order = order.resolve_type();

                if let Err(err) = book.append(order) {
                    panic!("{:#?}", err);
                }

                match book.try_match() {
                    Err(err) => println!("Match Failed {:?}\n\n", err),
                    Ok(transaction) => {
                        transactions.0.send(transaction).unwrap();
                    }
                }
            }
        })
        .unwrap();

    // Transaction publisher
    loop {
        match transactions.1.try_recv() {
            Ok(transaction) => {
                println!("Receive TRANSACTION in main: {:#?}\n\n", transaction);

                // Publish transaction to Kafka here
            }
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => {
                panic!("The channel has been disconnected, shutting down.");
            }
        }
    }
}
