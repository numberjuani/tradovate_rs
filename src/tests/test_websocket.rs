use tokio_tungstenite::tungstenite::Message;
use crate::client::TradovateClient;
use crate::websocket::requests::MarketData::*;
use crate::websocket::requests::MarketDataRequest;



fn on_message(message:String) {
    println!("Received message: {}", message);
}


#[tokio::test]
async fn test_market_data_socket() {
    let client = TradovateClient::load_from_env(crate::client::Server::Demo).authenticate().await.unwrap();
    let data_requests = vec![MarketDataRequest::new(Quote, "ESZ2"), MarketDataRequest::new(Chart, "NQZ2")];
    client.connect_to_market_data_socket(on_message,data_requests).await.unwrap();
}