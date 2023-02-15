use crate::data_utils::string_to_csv;



#[tokio::test]
async fn test_market_data_socket() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    use crate::models::{orderbook::new_orderbooks_rwl, time_and_sales::new_time_and_sales_rwl};
    let client = TradovateClient::load_from_env(crate::client::Server::Demo)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(DepthOfMarket, "ESH3"),
        MarketDataRequest::new(Chart, "ESH3"),
    ];
    let orderbooks = new_orderbooks_rwl();
    let time_and_sales = new_time_and_sales_rwl();
    client
        .connect_to_market_data_socket(data_requests,Some(30),orderbooks.clone(),time_and_sales.clone())
        .await
        .unwrap();
    assert!(time_and_sales.read().await.len() > 0);
    let mut csv = String::new();
    for book in orderbooks.read().await.iter() {
        csv.push_str(&book.to_csv_format());
    }
    string_to_csv(&csv, "test_book.csv");
    assert!(orderbooks.read().await.last().unwrap().doms.len() > 0);
}
