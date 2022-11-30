



#[tokio::test]
async fn test_auth_token() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client = client.authenticate().await.unwrap();
    println!("{:#?}",client.access_token_info);
    assert!(client.access_token_info.is_some())
}

#[tokio::test]
async fn test_contract_deps() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client = client.authenticate().await.unwrap();
    let deps = client.get_contract_deps().await;
    println!("{:#?}",deps);
    assert!(deps.is_ok())
}

#[tokio::test]
async fn test_products_list() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client = client.authenticate().await.unwrap();
    let list = client.get_products_list().await;
    println!("{:#?}",list);
    assert!(list.is_ok())
}


#[tokio::test]
async fn test_find_contract() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client = client.authenticate().await.unwrap();
    let contract = client.find_contract("ESZ2").await;
    println!("{:#?}",contract);
    assert!(contract.is_ok())
}

#[tokio::test]
async fn test_find_maturity() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Live);
    client = client.authenticate().await.unwrap();
    let maturity = client.find_maturity(46023).await;
    println!("{:#?}",maturity);
    assert!(maturity.is_ok())
}

#[tokio::test]
async fn test_positions() {
    let mut client = crate::client::TradovateClient::load_from_env(crate::client::Server::Demo);
    client = client.authenticate().await.unwrap();
    let positions = client.get_positions().await.unwrap();
    println!("{:#?}",positions);
    assert!(positions.len() > 0)
}