use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const POLYGON_WS_URI: &str = "wss://api.polygon.io/stocks";

pub async fn connect_crypto_stream(tx: mpsc::Sender<String>) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to the crypto stream!");

    let (write, read) = ws_stream.split();
    let read_stream = read.for_each(|message| async {
        if let Ok(msg) = message {
            if let Ok(text) = msg.into_text() {
                tx.send(text).await.expect("Failed to send message");
            }
        }
    });

    // Handle other part of the WebSocket stream or simply ignore it.
    // tokio::select! could be used here to handle multiple streams or timeout.

    read_stream.await;
}

pub async fn poll_stock_data(tx: mpsc::Sender<String>) {
    let mut interval = IntervalStream::new(interval(Duration::from_secs(10)));
    while interval.next().await.is_some() {
        // Example: Fetching stock data from an API
        let response = reqwest::get("https://api.example.com/stock_data")
            .await
            .expect("Failed to fetch data")
            .text()
            .await
            .expect("Failed to read response text");

        tx.send(response).await.expect("Failed to send stock data");
    }
}

pub async fn connect_polygon_stream(api_key: &str, symbols: Vec<String>) {
    let url = format!(
        "{}/v2/snapshot/locale/us/markets/stocks/tickers?apiKey={}",
        POLYGON_WS_URI, api_key
    );
    let url = Url::parse(&url).expect("Failed to parse URL");

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to Polygon WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to the symbols
    let subscribe_message = serde_json::json!({
        "action": "subscribe",
        "params": symbols.join(","),
    });

    write
        .send(Message::Text(subscribe_message.to_string()))
        .await
        .expect("Failed to send subscribe message");

    // Listen for messages
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    println!("Received message: {}", text);
                    // Process the message as needed
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}
