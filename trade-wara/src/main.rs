use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, TryRecvError},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, CommitMode, Consumer, DefaultConsumerContext},
    ClientConfig, Message,
};
use trade_wara::{
    entities::{
        order::{OrderItem, OrderResolution},
        transaction::Transaction,
    },
    order_book::OrderBook,
};

fn main() {
    let book_hash = Arc::new(Mutex::new(HashMap::new()));

    let orders = channel::<Arc<dyn OrderItem>>();
    let transactions = channel::<Arc<Transaction>>();

    thread::Builder::new()
        .name("kafka-listener".into())
        .spawn(move || {
            let consumer = ClientConfig::new()
                .set("group.id", "rust_consumer_group")
                .set("bootstrap.servers", "localhost:19092")
                .set("enable.partition.eof", "false")
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "true")
                .set_log_level(RDKafkaLogLevel::Debug)
                .create::<BaseConsumer<DefaultConsumerContext>>()
                .expect("Failed to create consumer");

            consumer
                .subscribe(&["test_topic"])
                .expect("Failed to subscribe");

            // List Kafka here
            loop {
                match consumer.poll(Duration::ZERO) {
                    Some(msg) => {
                        println!("message received");

                        let msg = msg.expect("Failed to get message");

                        println!("MESSAGE : {:?}", msg);

                        let payload = msg
                            .payload()
                            .expect("Failed to get message payload");

                        let order: OrderResolution =
                            serde_json::from_slice(payload)
                                .expect("Failed to parse message payload");

                        let order: Box<dyn OrderItem> = order.into();

                        if let Ok(()) = orders.0.send(order.into()) {
                            consumer
                                .commit_message(&msg, CommitMode::Sync)
                                .unwrap();
                        }
                    }
                    None => (),
                }
            }
        })
        .unwrap();

    thread::Builder::new()
        .name("trade-matcher".into())
        .spawn(move || loop {
            if let Ok(order) = orders.1.try_recv() {
                let mut book_hash = book_hash.lock().unwrap();

                let book = book_hash
                    .entry(order.asset_id())
                    .or_insert(OrderBook::new(order.asset_id().to_string()));

                let order = order.resolve_type();

                println!("Received order: {:#?}", order);

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
