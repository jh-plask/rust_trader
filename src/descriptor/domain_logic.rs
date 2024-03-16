mod domain_entity;

impl domain_entity::Asset {
    pub fn new(symbol: String, name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            symbol,
            name,
        }
    }
}

pub impl<T: Tradable> Order<T> {
    pub fn new(
        account_id: uuid::Uuid,
        asset: T,
        market_id: String,
        order_type: OrderType,
        quantity: f64,
        price: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            account_id,
            asset,
            market_id,
            order_type,
            status: OrderStatus::Open,
            quantity,
            price,
            timestamp: chrono::Utc::now().naive_utc(),
        }
    }
}
