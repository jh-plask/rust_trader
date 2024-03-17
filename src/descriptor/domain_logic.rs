use super::domain_entity::{Asset, Order, OrderStatus, OrderType};
use rust_decimal::Decimal;
use uuid::Uuid;

impl Asset {
    pub fn new(symbol: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol,
            name,
        }
    }
}

impl Order {
    pub fn new(
        account_id: uuid::Uuid,
        asset_id: uuid::Uuid,
        market_id: String,
        order_type: OrderType,
        quantity: Decimal,
        price: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            market_id,
            order_type,
            status: OrderStatus::Open(chrono::Utc::now()),
            quantity,
            price,
        }
    }
}
