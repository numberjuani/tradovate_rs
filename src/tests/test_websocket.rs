

#[tokio::test]
async fn test_market_data_socket() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    use crate::models::{orderbook::new_orderbooks_rwl, time_and_sales::new_time_and_sales_rwl};
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    let client = TradovateClient::load_from_env(crate::client::Server::Demo)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(DepthOfMarket, "ESH3"),
        MarketDataRequest::new(Chart, "ESH3"),
        MarketDataRequest::new(Quotes, "ESH3"),
    ];
    let orderbooks = new_orderbooks_rwl();
    let time_and_sales = new_time_and_sales_rwl();
    client
        .connect_to_market_data_socket(&data_requests,orderbooks.clone(),time_and_sales.clone())
        .await
        .unwrap();
    assert!(time_and_sales.read().await.len() > 0);
    assert!(orderbooks.read().await.last().unwrap().doms.len() > 0);
}


#[tokio::test]
async fn test_market_replay() {
    use crate::client::TradovateClient;
    use crate::websocket::requests::MarketData::*;
    use crate::websocket::requests::MarketDataRequest;
    use crate::models::{orderbook::new_orderbooks_rwl, time_and_sales::new_time_and_sales_rwl};
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    let start_date = chrono::NaiveDate::from_ymd_opt(2022, 9, 15).unwrap().and_hms_opt(0, 00, 0).unwrap();
    let start_date = chrono::DateTime:: <chrono::Utc> ::from_utc(start_date, chrono::Utc);
    let end_date = chrono::NaiveDate::from_ymd_opt(2022, 9, 15).unwrap().and_hms_opt(0, 01, 0).unwrap();
    let end_date = chrono::DateTime:: <chrono::Utc> ::from_utc(end_date, chrono::Utc);
    let settings = crate::websocket::market_replay::MarketReplaySettings{ start_timestamp: start_date, speed: 400, initial_balance: 51000 };
    let client = TradovateClient::load_from_env(crate::client::Server::Live)
        .authenticate()
        .await
        .unwrap();
    let data_requests = vec![
        MarketDataRequest::new(DepthOfMarket, "ESZ2"),
        MarketDataRequest::historical_chart(Chart, "ESZ2",start_date),
        MarketDataRequest::new(Quotes, "ESZ2"),
    ];
    let orderbooks = new_orderbooks_rwl();
    let time_and_sales = new_time_and_sales_rwl();
    let quotes = crate::models::quotes::new_quotes_rwl();
    client
        .connect_to_market_replay(&data_requests,&settings,orderbooks.clone(),time_and_sales.clone(),quotes.clone(),end_date)
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

#[tokio::test]
async fn test_deserialize() {
    let test = "Clock";
    println!("{:#?}",serde_json::to_string(&crate::websocket::requests::MarketData::Clock).unwrap());
    let data = serde_json::from_str::<crate::websocket::requests::MarketData>(test).unwrap();
    println!("{:?}",data);
}

// #[tokio::test]
// async fn test_clock() {
    
//     match serde_json::from_value::<crate::models::replay_clock::ReplayClock>(json_data["d"].clone()) {
//         Ok(p_clock) => {
//             println!("clock: {:?}", p_clock);
//             Ok(())
//         },
//         Err(e) => {
//             error!("error parsing clock data: {}", e);
//             Err(TradovateWSError::ParseError(e))
//         }
//     }
// }