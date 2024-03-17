use rust_decimal::Decimal;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    username: String,
    email: String,
}

pub struct Account {
    id: Uuid,
    user_id: uuid::Uuid,
    balance: Decimal,
    currency: String,
}

pub enum OrderType {
    Buy,
    Sell,
}

pub enum OrderStatus {
    Open(chrono::DateTime<chrono::Utc>),
    PartiallyFilled(chrono::DateTime<chrono::Utc>),
    Filled(chrono::DateTime<chrono::Utc>),
    Cancelled(chrono::DateTime<chrono::Utc>),
}

pub struct Order {
    pub id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub market_id: String, // Identifies the market (e.g., "BTC-USD")
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub quantity: Decimal,
    pub price: Decimal,
}

pub struct Trade {
    id: uuid::Uuid,
    buy_order_id: uuid::Uuid,
    sell_order_id: uuid::Uuid,
    quantity: f64, // Consider using a decimal type
    price: f64,    // Consider using a decimal type
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct Asset {
    pub id: uuid::Uuid,
    pub symbol: String, // ticker-like symbol e.g., "BTC"
    pub name: String,   // e.g., "Bitcoin"
}

pub enum MarketEvent {
    // environment change
}
