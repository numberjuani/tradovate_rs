

use crate::models::orders::OrderTicket;

#[tokio::test]
async fn test_auth_token() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    println!("{:#?}", client.access_token_info);
    assert!(client.access_token_info.is_some())
}

#[tokio::test]
async fn test_contract_deps() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    let deps = client.get_contract_deps().await;
    println!("{:#?}", deps);
    assert!(deps.is_ok())
}

#[tokio::test]
async fn test_products_list() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    let list = client.get_products_list().await;
    println!("{:#?}", list);
    assert!(list.is_ok())
}

#[tokio::test]
async fn test_find_contract() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    let contract = client.find_contract("ESZ2").await;
    println!("{:#?}", contract);
    assert!(contract.is_ok())
}

#[tokio::test]
async fn test_find_maturity() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    let maturity = client.find_maturity(46023).await;
    println!("{:#?}", maturity);
    assert!(maturity.is_ok())
}

#[tokio::test]
async fn test_positions() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Demo);
    client.authenticate().await.unwrap();
    let positions = client.get_positions().await;
    println!("{:#?}", positions);
    assert!(positions.is_ok())
}

#[tokio::test]
async fn test_place_order() {
    //set env logger to debug
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Demo);
    client.authenticate().await.unwrap();
    let balances = client.get_cash_balances().await.unwrap();
    let order = client.place_order(OrderTicket::market_sell(&client.username,balances[0].account_id,"ESH3", 1)).await;
    println!("{:#?}", order);
    assert!(order.is_ok())
}


#[tokio::test]
async fn test_accounts_list() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client.authenticate().await.unwrap();
    let accounts = client.get_accounts_list().await;
    println!("{:#?}", accounts);
    assert!(accounts.is_ok())
}

#[tokio::test]
async fn test_balance_list() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Demo);
    client.authenticate().await.unwrap();
    let balances = client.get_cash_balances().await;
    println!("{:#?}", balances);
    assert!(balances.is_ok())
}