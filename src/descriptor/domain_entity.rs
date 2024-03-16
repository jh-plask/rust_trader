pub struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
}

pub struct Account {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    balance: f64, // Consider using a decimal type for financial applications
    currency: String,
}

pub enum OrderType {
    Buy,
    Sell,
}

pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}

pub trait Tradable {
    fn ticker(&self) -> String;
    fn current_price(&self) -> f64;
    // Additional methods can be defined here.
}
pub trait Executable {
    fn execute(&self, order: &Order<Self>)
    where
        Self: Sized;
    // More detailed execution strategies can be defined.
}

pub struct Order<T: Tradable> {
    id: uuid::Uuid,
    account_id: uuid::Uuid,
    asset: T,
    market_id: String, // Identifies the market (e.g., "BTC-USD")
    order_type: OrderType,
    status: OrderStatus,
    quantity: f64,                    // Consider using a decimal type
    price: f64,                       // Consider using a decimal type
    timestamp: chrono::NaiveDateTime, // Timestamp for order creation
}

pub struct Trade {
    id: uuid::Uuid,
    buy_order_id: uuid::Uuid,
    sell_order_id: uuid::Uuid,
    quantity: f64,                    // Consider using a decimal type
    price: f64,                       // Consider using a decimal type
    timestamp: chrono::NaiveDateTime, // Timestamp for when the trade was executed
}

pub struct MarketData {
    market_id: String,                // e.g., "BTC-USD"
    price: f64,                       // Consider using a decimal type
    volume_24h: f64,                  // 24h trading volume
    change_24h: f64,                  // 24h price change
    high_24h: f64,                    // 24h high price
    low_24h: f64,                     // 24h low price
    timestamp: chrono::NaiveDateTime, // Latest update timestamp
}

pub struct Asset {
    id: uuid::Uuid,
    symbol: String, // ticker-like symbol e.g., "BTC"
    name: String,   // e.g., "Bitcoin"
}
