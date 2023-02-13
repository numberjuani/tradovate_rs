



#[tokio::test]
async fn test_market_data_socket() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    use crate::websocket::process_message::parse_messages;
    let client = TradovateClient::load_from_env(crate::client::Server::Demo)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(DepthOfMarket, "ESH3"),
    ];
    client
        .connect_to_market_data_socket(parse_messages, data_requests)
        .await
        .unwrap();
}
