# Trade Wara - The Match Service

![Match Service Architecture](<path-to-your-match-service-architecture-diagram>)

The Match Service is a critical component of the Trading platform written in Rust. Its primary responsibility is to match buy and sell orders in real-time using a **min/max heap algorithm**. The service plays a central role in ensuring efficient order matching and determining the platform gain. Below is an overview of the key components and interactions of the Match Service:

### Domain Rules:

1. **Order Matching:**
  The Match Service applies the "price-time priority" rule, meaning that if multiple orders have the same price, the order placed earlier takes precedence. This ensures fairness and consistency in executing trades.

2. **Platform Gain Calculation:**
  After a trade is executed, the Match Service calculates the platform gain based on the price difference between the matched buy and sell orders. Any applicable fees or commissions are considered in the gain calculation.

### Components:

1. **Order Book:**
  The Order Book is a data structure that stores all active buy and sell orders. It consists of two binary heaps - one for buy orders (max heap) and another for sell orders (min heap). Each order is represented as a node in the respective heap.

2. **Buy Orders Heap (Max Heap):**
  The Buy Orders Heap is a binary max heap that stores all active buy orders placed by traders. The order with the highest price is placed at the root of the heap, making it readily accessible for quick retrieval.

3. **Sell Orders Heap (Min Heap):**
  The Sell Orders Heap is a binary min heap that stores all active sell orders placed by traders. The order with the lowest price is placed at the root of the heap, ensuring quick access for matching.

4. **Matching Algorithm:**
  The Matching Algorithm is a key domain logic implemented in the Match Service. It continuously checks for matching buy and sell orders and executes trades when conditions are met. The algorithm compares the highest buy order's price with the lowest sell order's price to determine if a trade can be executed.

### Algorithms:

1. **Min/Max Heap Algorithm:**
  The Min/Max Heap Algorithm is employed to keep the buy and sell orders efficiently sorted by their prices. This enables constant-time access to the highest buy order and the lowest sell order for matching.

### Data Structures:

1. **Binary Heaps:**
  Binary heaps are used to implement the Buy Orders Heap (max heap) and the Sell Orders Heap (min heap). These data structures efficiently maintain the order book's sorted nature, allowing for efficient insertion and extraction of orders.

2. **Order Data Structure:**
  Each buy and sell order is represented by an Order Data Structure. This structure contains relevant information, such as order ID, user ID, price, quantity, and timestamp, required for order matching and execution.

3. **HashMap (Ticker to Order Book):**
  The HashMap data structure is utilized to organize the order books corresponding to different share tickers. Each share ticker maps to an Order Book, which stores all active buy and sell orders for that particular share. This approach enables quick access to the order book of a specific share, enhancing the overall efficiency of the platform.


With the Match Service's architecture, domain rules, algorithms, and data structures clearly defined, the platform ensures efficient and fair order matching, empowering traders to make informed decisions and participate in real-time trading activities.
