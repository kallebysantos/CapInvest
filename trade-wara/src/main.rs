use std::{
    collections::BinaryHeap,
    sync::{
        mpsc::{channel, TryRecvError},
        Arc,
    },
    thread,
};

use trade_wara::entities::order::{
    Buy, Open, Order, OrderItem, OrderResolution, OrderTransition, Sell,
};

fn main() {
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
            let mut buy_orders = BinaryHeap::<Order<Buy, Open>>::new();
            let mut sell_orders = BinaryHeap::<Order<Sell, Open>>::new();

            if let Ok(order) = orders_in.1.try_recv() {
                match order.resolve_type() {
                    OrderResolution::Sell(order) => {
                        let OrderTransition::Open(order) = order else {
                            panic!("Order is already closed");
                        };

                        println!("SELL: {:?}", order);
                        sell_orders.push(order);
                    }
                    OrderResolution::Buy(order) => {
                        let OrderTransition::Open(order) = order else {
                            panic!("Order is already closed");
                        };

                        println!("BUY: {:?}", order);
                        buy_orders.push(order);
                    }
                }
            }
            // Send order to orders_out
        })
        .unwrap();

    // Order publisher
    loop {
        match orders_out.1.try_recv() {
            Ok(_) => println!("Receive order in main"), // Publish processed order Kafka here
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => {
                panic!("The channel has been disconnected, shutting down.");
            }
        }
    }
}
