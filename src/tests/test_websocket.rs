use crate::{data_utils::string_to_csv};



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
        .connect_to_market_data_socket(&data_requests,orderbooks.clone(),time_and_sales.clone(),"15:15")
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


#[tokio::test]
async fn test_market_replay() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    use crate::models::{orderbook::new_orderbooks_rwl, time_and_sales::new_time_and_sales_rwl};
    let naivedatetime_utc = chrono::NaiveDate::from_ymd_opt(2022, 9, 01).unwrap().and_hms_opt(0, 00, 0).unwrap();
    let datetime_utc = chrono::DateTime:: <chrono::Utc> ::from_utc(naivedatetime_utc, chrono::Utc);
    let settings = crate::websocket::market_replay::MarketReplaySettings{ start_timestamp: datetime_utc, speed: 400, initial_balance: 51000 };
    let client = TradovateClient::load_from_env(crate::client::Server::Live)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(DepthOfMarket, "ESZ2"),
        MarketDataRequest::historical_chart(Chart, "ESZ2",datetime_utc),
        //MarketDataRequest::new(Quote, "ESZ2"),
    ];
    let orderbooks = new_orderbooks_rwl();
    let time_and_sales = new_time_and_sales_rwl();
    client
        .connect_to_market_replay(&data_requests,&settings,orderbooks.clone(),time_and_sales.clone(),"20:00")
        .await
        .unwrap();
    assert!(time_and_sales.read().await.len() > 0);
    assert!(orderbooks.read().await.last().unwrap().doms.len() > 0);
}

#[tokio::test]
async fn test_market_replay_settings() {
    let naivedatetime_utc = chrono::NaiveDate::from_ymd_opt(2019, 8, 26).unwrap().and_hms_opt(16, 43, 0).unwrap();
    let datetime_utc = chrono::DateTime:: <chrono::Utc> ::from_utc(naivedatetime_utc, chrono::Utc);
    let settings = crate::websocket::market_replay::MarketReplaySettings{ start_timestamp: datetime_utc, speed: 20, initial_balance: 51000 };
    let request = settings.to_request(3);
    println!("{}",request);
}
