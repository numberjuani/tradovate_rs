use log::debug;
use reqwest::header;
use serde_json::{json, Value};

use crate::{
    constants::{AUTH_FILENAME, DEMO_TRADING_URL, LIVE_MARKET_DATA_URL, LIVE_TRADING_URL, MARKET_REPLAY_WS},
    error::Error,
    models::{
        access_token::AccessTokenInfo,
        contract::{Contract, Maturity},
        product::Product, position::Position, orders::OrderTicket, account::Balances,
    },
    rest::endpoints::{Endpoint, CONTRACT_DEPS, CONTRACT_FIND, CONTRACT_MATURITY, PRODUCTS_LIST, LIST_POSITIONS, PLACE_ORDER, ACCOUNTS_LIST, CASH_BALANCE_LIST},
    utils::delete_file,
};

#[derive(Debug, Clone)]
pub enum Server {
    Live,
    Demo,
}
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Trading,
    MarketData,
    MarketReplay
}
pub enum Protocol {
    Https,
    Wss,
}
impl Protocol {
    pub fn add_prefix(&self, base_url: &str) -> String {
        match self {
            Protocol::Https => format!("https://{}", base_url),
            Protocol::Wss => url::Url::parse(&format!("wss://{}/v1/websocket", base_url))
                .unwrap()
                .to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradovateClient {
    pub server_type: Server,
    pub username: String,
    pub password: String,
    pub app_id: String,
    pub app_version: String,
    cid: i64,
    secret: String,
    pub access_token_info: Option<AccessTokenInfo>,
    pub http_client: reqwest::Client,
}
impl TradovateClient {
    /// The `TradovateClient` struct contains all the necessary information to make requests to Tradovate's api.
    pub fn new(
        server_type: Server,
        app_id: &str,
        app_version: &str,
        cid: i64,
        secret: String,
        username: String,
        password: String,
    ) -> Self {
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert("Content-Type", "application/json".parse().unwrap());
        default_headers.insert("Accept", "application/json".parse().unwrap());
        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();
        Self {
            app_id: app_id.to_string(),
            app_version: app_version.to_string(),
            access_token_info: None,
            server_type,
            cid,
            secret,
            username,
            password,
            http_client: client,
        }
    }
    /// This function will load the necessary values from the user's environment variables.
    /// Can be used with .env files.
    /// Make sure the envs are set before calling this function.
    /// `TRADOVATE_APP_ID` - The app's name you selected when creating API keys
    /// `TRADOVATE_APP_VERSION` - The app version provided by Tradovate
    /// `TRADOVATE_USERNAME` - Your tradovate username
    /// `TRADOVATE_PASSWORD` - Your tradovate password
    /// `TRADOVATE_CID` - The cid provided by Tradovate
    /// `TRADOVATE_SECRET` - The secret provided by Tradovate
    pub fn load_from_env(server_type: Server) -> Self {
        dotenv::dotenv().ok();
        let app_id = std::env::var("TRADOVATE_APP_ID").expect("TRADOVATE_APP_ID must be set");
        let app_version =
            std::env::var("TRADOVATE_APP_VERSION").expect("TRADOVATE_APP_VERSION must be set");
        let cid = std::env::var("TRADOVATE_CID").expect("TRADOVATE_CID must be set");
        let cid = cid.parse::<i64>().unwrap();
        let secret = std::env::var("TRADOVATE_SECRET").expect("TRADOVATE_SECRET must be set");
        let username = std::env::var("TRADOVATE_USERNAME").expect("TRADOVATE_USERNAME must be set");
        let password = std::env::var("TRADOVATE_PASSWORD").expect("TRADOVATE_PASSWORD must be set");
        Self::new(
            server_type,
            &app_id,
            &app_version,
            cid,
            secret,
            username,
            password,
        )
    }
    fn get_auth_data(&self) -> Value {
        json!({
            "name":       self.username,
            "password":   self.password,
            "appId":      self.app_id,
            "appVersion": self.app_version,
            "cid":        self.cid,
            "sec":        self.secret,
            "deviceId":   machine_uid::get().unwrap_or("buster-linux-docker".to_string())
        })
    }
    pub fn url(&self, resource_type: ResourceType, protocol: Protocol) -> String {
        match self.server_type {
            Server::Live => match resource_type {
                ResourceType::Trading => protocol.add_prefix(LIVE_TRADING_URL),
                ResourceType::MarketData => protocol.add_prefix(LIVE_MARKET_DATA_URL),
                ResourceType::MarketReplay => protocol.add_prefix(MARKET_REPLAY_WS),
            },
            Server::Demo => match resource_type {
                ResourceType::Trading => protocol.add_prefix(DEMO_TRADING_URL),
                ResourceType::MarketData => protocol.add_prefix(LIVE_MARKET_DATA_URL),
                ResourceType::MarketReplay => protocol.add_prefix(MARKET_REPLAY_WS),
            },
        }
    }
    async fn call_endpoint(
        &self,
        endpoint: Endpoint,
        params: Option<Value>,
        request_body: Option<Value>,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}{}",
            self.url(ResourceType::Trading, Protocol::Https),
            endpoint.path
        );
        let mut request = self.http_client.request(endpoint.method, url);
        if let Some(params) = params {
            request = request.query(&params);
        }
        if let Some(request_body) = request_body {
            request = request.json(&request_body);
        }
        if let Some(access_token_info) = &self.access_token_info {
            request = request.bearer_auth(&access_token_info.access_token);
        }
        let response = request.send().await?;
        debug!("Response: {:?}", response);
        Ok(response.text().await?)
    }
    pub fn ws_auth_msg(&self) -> String {
        format!("authorize\n1\n\n{}", self.access_token_info.as_ref().unwrap().access_token)
    }
    async fn get_access_token(mut self) -> Result<Self, Error> {
        use crate::rest::endpoints::ACCESS_TOKEN;
        match self
            .call_endpoint(ACCESS_TOKEN, None, Some(self.get_auth_data()))
            .await
        {
            Ok(access_token_info) => {
                match serde_json::from_str::<AccessTokenInfo>(&access_token_info) {
                    Ok(access_token_info) => {
                        self.access_token_info = Some(access_token_info);
                        Ok(self)
                    }
                    Err(e) => Err(Error::Json(e)),
                }
            }
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    /// This function will return a new instance of the client with an access token.
    /// It will either load the token from the auth file, check that its still valid, or
    /// request a new one.
    pub async fn authenticate(mut self) -> Result<Self, Error> {
        match crate::utils::open_json(AUTH_FILENAME) {
            Ok(access_token_info) => {
                match serde_json::from_value::<AccessTokenInfo>(access_token_info) {
                    Ok(access_token_info) => {
                        if access_token_info.is_expired() {
                            delete_file(AUTH_FILENAME);
                            let client = self.get_access_token().await?;
                            crate::utils::create_json_file(
                                AUTH_FILENAME,
                                &client.access_token_info,
                            );
                            Ok(client)
                        } else {
                            self.access_token_info = Some(access_token_info);
                            Ok(self)
                        }
                    }
                    Err(e) => Err(Error::Json(e)),
                }
            }
            Err(_) => {
                let client = self.get_access_token().await?;
                crate::utils::create_json_file(AUTH_FILENAME, &client.access_token_info);
                Ok(client)
            }
        }
    }
    pub async fn get_contract_deps(&self) -> Result<String, Error> {
        match self.call_endpoint(CONTRACT_DEPS, None, None).await {
            Ok(contract_deps) => Ok(contract_deps),
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn get_products_list(&self) -> Result<Vec<Product>, Error> {
        match self.call_endpoint(PRODUCTS_LIST, None, None).await {
            Ok(products_list) => match serde_json::from_str::<Vec<Product>>(&products_list) {
                Ok(products_list) => Ok(products_list),
                Err(e) => Err(Error::Json(e)),
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn find_contract(&self, name: &str) -> Result<Contract, Error> {
        let params = json!({ "name": name });
        match self.call_endpoint(CONTRACT_FIND, Some(params), None).await {
            Ok(contract_str) => match serde_json::from_str::<Contract>(&contract_str) {
                Ok(contract) => Ok(contract),
                Err(e) => Err(Error::Json(e)),
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn find_maturity(&self, id: i64) -> Result<Maturity, Error> {
        let params = json!({ "id": id });
        match self
            .call_endpoint(CONTRACT_MATURITY, Some(params), None)
            .await
        {
            Ok(contract_mat) => match serde_json::from_str::<Maturity>(&contract_mat) {
                Ok(mat) => Ok(mat),
                Err(e) => Err(Error::Json(e)),
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn get_positions(&self) -> Result<Vec<Position>,Error> {
        match self.call_endpoint(LIST_POSITIONS, None, None).await {
            Ok(positions) => {
                match serde_json::from_str::<Vec<Position>>(&positions) {
                    Ok(positions) => Ok(positions),
                    Err(e) => Err(Error::Json(e)),
                }
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn place_order(&self,order_ticket:OrderTicket) -> Result<Value,Error> {
        let value = json!(order_ticket);
        debug!("{}",serde_json::to_string_pretty(&value).unwrap());
        match self.call_endpoint(PLACE_ORDER, None, Some(value)).await {
            Ok(order) => {
                debug!("{}",order);
                match serde_json::from_str::<Value>(&order) {
                    Ok(order) => Ok(order),
                    Err(e) => Err(Error::Json(e)),
                }
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn get_accounts_list(&self) -> Result<Value,Error> {
        match self.call_endpoint(ACCOUNTS_LIST, None, None).await {
            Ok(order) => {
                match serde_json::from_str::<Value>(&order) {
                    Ok(order) => Ok(order),
                    Err(e) => Err(Error::Json(e)),
                }
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
    pub async fn get_cash_balances(&self) -> Result<Balances,Error> {
        match self.call_endpoint(CASH_BALANCE_LIST, None, None).await {
            Ok(order) => {
                match serde_json::from_str::<Balances>(&order) {
                    Ok(order) => Ok(order),
                    Err(e) => Err(Error::Json(e)),
                }
            },
            Err(e) => Err(Error::Reqwest(e)),
        }
    }
}
