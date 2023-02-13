#[tokio::test]
async fn test_market_data_socket() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    fn on_message(message: String) {
        println!("Received message: {}", message);
    }
    let client = TradovateClient::load_from_env(crate::client::Server::Demo)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(Quote, "ESH3"),
        MarketDataRequest::new(Chart, "NQH3"),
    ];
    client
        .connect_to_market_data_socket(on_message, data_requests)
        .await
        .unwrap();
}
