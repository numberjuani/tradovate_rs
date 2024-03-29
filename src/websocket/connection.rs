use std::{sync::Arc};

use crate::{
    client::{Protocol, ResourceType, TradovateClient},
    models::{orderbook::OrderBooksRWL, quotes::QuotesRWL, time_and_sales::TimeAndSalesRWL},
    websocket::market_replay::replay_messages,
};
use chrono::{DateTime, Utc};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{error, info, warn, debug};
use serde_json::json;
use tokio::{net::TcpStream, sync::{Notify, Mutex}};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
pub type WriteWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type ReadWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
use super::{
    market_replay::MarketReplaySettings, process_message::parse_messages,
    requests::MarketDataRequest,
};
use crate::websocket::connection::Message::Text;
use crate::models::user_data::UserSyncMessage;
pub async fn keep_listening(
    mut reader: ReadWs,
    orderbooks_rwl: OrderBooksRWL,
    time_and_sales_rwl: TimeAndSalesRWL,
    notify: Arc<Notify>,
) -> Result<(), Error> {
    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => match msg {
                Message::Text(txtmsg) => {
                    if let Err(e) = parse_messages(
                        txtmsg,
                        orderbooks_rwl.clone(),
                        time_and_sales_rwl.clone(),
                        notify.clone(),
                    )
                    .await
                    {
                        error!("Error in websocket {:#?}", e);
                        return Err(Error::ConnectionClosed);
                    }
                }
                Message::Close(_) => {
                    warn!("Received close message");
                    return Ok(());
                }
                _ => {
                    error!("Received unexpected message: {:?}", msg);
                }
            },
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub async fn keep_listening_account(mut reader: ReadWs) -> Result<(), Error> {
    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => match msg {
                Message::Text(txtmsg) => {
                    if txtmsg.len() < 3 {
                        continue;
                    }
                    let txtmsg = &txtmsg[2..txtmsg.len()-1];
                    match serde_json::from_str::<UserSyncMessage>(&txtmsg) {
                        Ok(acc) => {
                            debug!("Received user sync message {:#?}",acc);
                        }
                        Err(_) => {
                            error!("COuld not parse message: {}", txtmsg);
                        }
                    }
                }
                Message::Close(_) => {
                    warn!("Received close message");
                    return Ok(());
                }
                _ => {
                    error!("Received unexpected message: {:?}", msg);
                }
            },
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub async fn send_heartbeats(mut writer: WriteWs) -> Result<(), Error> {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(2502));
    loop {
        interval.tick().await;
        writer.send(Text(String::from("[]"))).await?;
    }
}

pub async fn send_heartbeats_acc(writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>) -> Result<(), Error> {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(2502));
    loop {
        interval.tick().await;
        writer.lock().await.send(Text(String::from("[]"))).await?;
    }
}

impl TradovateClient {
    pub async fn connect_to_market_data_socket(
        &self,
        requests: &[MarketDataRequest],
        orderbooks_rwl: OrderBooksRWL,
        time_and_sales_rwl: TimeAndSalesRWL,
        notify: Arc<Notify>,
    ) -> Result<(), Error> {
        let url = self.url(ResourceType::MarketData, Protocol::Wss);
        let (ws_stream, response) = tokio_tungstenite::connect_async(url).await?;
        info!(
            "Connected to market data socket, status {:#?}",
            response.status()
        );
        let (mut write, reader) = ws_stream.split();
        let mut string_requests = Vec::new();
        write.send(Text(self.ws_auth_msg())).await?;
        for (index, request) in requests.iter().enumerate() {
            string_requests.push(request.subscribe(index + 2));
        }
        for string_request in string_requests {
            println!("{}",string_request);
            write.send(Text(string_request)).await?;
        }
        tokio::select!(
            biased;
            listen_result = tokio::spawn(keep_listening(reader,orderbooks_rwl,time_and_sales_rwl,notify.clone())) => {
                if let Err(e) = listen_result.unwrap() {
                    error!("Error in websocket {:#?}", e);
                    return Err(Error::ConnectionClosed);
                }
            },
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
    pub async fn connect_to_account_socket(&self,order_receive: tokio::sync::mpsc::Receiver<std::string::String>) -> Result<(), Error> {
        let url = self.url(ResourceType::Trading, Protocol::Wss);
        let (ws_stream, response) = tokio_tungstenite::connect_async(url).await?;
        info!(
            "Connected to market data socket, status {:#?}",
            response.status()
        );
        let (mut write, reader) = ws_stream.split();
        write.send(Text(self.ws_auth_msg())).await?;
        write.send(Text(self.get_user_sync_request(1))).await?;
        let sender = Arc::new(Mutex::new(write));
        tokio::select!(
            biased;
            listen_result = tokio::spawn(keep_listening_account(reader)) => {
                if let Err(e) = listen_result.unwrap() {
                    error!("Error in websocket {:#?}", e);
                    return Err(Error::ConnectionClosed);
                }
            },
            _ = tokio::spawn(send_heartbeats_acc(sender.clone())) => {
                info!("Heartbeats stopped");
            },
            _ =  tokio::spawn(send_orders(order_receive,sender)) => {

            }
        );
        Ok(())
    }
    pub async fn connect_to_market_replay(
        &self,
        requests: &[MarketDataRequest],
        settings: &MarketReplaySettings,
        orderbooks_rwl: OrderBooksRWL,
        time_and_sales_rwl: TimeAndSalesRWL,
        quotes: QuotesRWL,
        end_datetime: DateTime<Utc>,
    ) -> Result<(), Error> {
        let url = self.url(ResourceType::MarketReplay, Protocol::Wss);
        let (ws_stream, response) = tokio_tungstenite::connect_async(&url).await?;
        info!("Connected to {url}, status {:#?}", response.status());
        let (mut write, reader) = ws_stream.split();
        write.send(Text(self.ws_auth_msg())).await?;
        write.send(Text(settings.to_request(2))).await?;
        for (index, request) in requests.iter().enumerate() {
            let string_request = request.subscribe(index + 3);
            write.send(Text(string_request)).await?;
        }
        tokio::select!(
            biased;
            listen_result = tokio::spawn(replay_messages(reader,orderbooks_rwl,time_and_sales_rwl,quotes,end_datetime)) => {
                if let Err(e) = listen_result.unwrap() {
                    error!("Error in websocket {:#?}", e);
                    return Err(Error::ConnectionClosed);
                }
            },
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
    pub fn get_user_sync_request(&self, request_id: usize) -> String {
        let body = json!({"users":[self.access_token_info.as_ref().unwrap().user_id]});
        let request = format!("user/syncrequest\n{}\n\n{}", request_id, body);
        request
    }
}


pub async fn send_orders(mut order_receive: tokio::sync::mpsc::Receiver<std::string::String>,sender: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>){
    while let Some(res) = order_receive.recv().await {
        sender.lock().await.send(Text(res)).await.unwrap();
    }
}