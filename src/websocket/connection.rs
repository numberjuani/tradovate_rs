

use crate::{client::{Protocol, ResourceType, TradovateClient}, models::{orderbook::OrderBooksRWL, time_and_sales::TimeAndSalesRWL}, time_utils::calculate_seconds_to_cst_time};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{error, info, warn};
use tokio::{net::TcpStream};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
pub type WriteWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type ReadWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
use super::{requests::MarketDataRequest, process_message::parse_messages, market_replay::MarketReplaySettings};
use crate::websocket::connection::Message::Text;

pub async fn keep_listening(
    mut reader:ReadWs,
    orderbooks_rwl: OrderBooksRWL,
    time_and_sales_rwl: TimeAndSalesRWL,
) -> Result<(), Error> {
    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => {
                match msg {
                    Message::Text(txtmsg) => {
                        if let Err(e) = parse_messages(txtmsg, orderbooks_rwl.clone(), time_and_sales_rwl.clone()).await {
                            error!("Error in websocket {:#?}", e);
                            return Err(Error::ConnectionClosed);
                        }
                    },
                    Message::Close(_) => {
                        warn!("Received close message");
                        return Ok(());
                    }
                    _ => {
                        error!("Received unexpected message: {:?}", msg);
                    },
                }
            }
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

impl TradovateClient {
    pub async fn connect_to_market_data_socket(
        &self,
        requests: &Vec<MarketDataRequest>,
        orderbooks_rwl: OrderBooksRWL,
        time_and_sales_rwl: TimeAndSalesRWL,
        disconnect_time:&str
    ) -> Result<(), Error> {
        let disconnect_split = disconnect_time.split(":").collect::<Vec<&str>>();
        let seconds_to_disconnect = calculate_seconds_to_cst_time(disconnect_split[0].parse::<u32>().unwrap(),disconnect_split[1].parse::<u32>().unwrap());
        let url = self.url(ResourceType::MarketData, Protocol::Wss);
        let (ws_stream, response) = tokio_tungstenite::connect_async(url).await?;
        info!("Connected to market data socket, status {:#?}", response.status());
        let (mut write, reader) = ws_stream.split();
        write.send(Text(self.ws_auth_msg())).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        for (index, request) in requests.iter().enumerate() {
            write.send(Text(request.subscribe(index + 2))).await?;
        }
        let timeout = tokio::time::sleep(std::time::Duration::from_secs(seconds_to_disconnect));
        tokio::pin!(timeout);
        tokio::select!(
            biased;
            _ = &mut timeout => {
                info!("timeout");
                return Ok(());
            }
            listen_result = tokio::spawn(keep_listening(reader,orderbooks_rwl,time_and_sales_rwl)) => {
                if let Err(e) = listen_result.unwrap() {
                    error!("Error in websocket {:#?}", e);
                    return Err(Error::ConnectionClosed);
                }
            },
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
    pub async fn connect_to_market_replay(
        &self,
        requests: &Vec<MarketDataRequest>,
        settings: &MarketReplaySettings,
        orderbooks_rwl: OrderBooksRWL,
        time_and_sales_rwl: TimeAndSalesRWL,
        disconnect_time:&str
    ) -> Result<(), Error> {
        let disconnect_split = disconnect_time.split(":").collect::<Vec<&str>>();
        let seconds_to_disconnect = calculate_seconds_to_cst_time(disconnect_split[0].parse::<u32>().unwrap(),disconnect_split[1].parse::<u32>().unwrap());
        let url = self.url(ResourceType::MarketReplay, Protocol::Wss);
        println!("url: {:#?}", url);
        let (ws_stream, response) = tokio_tungstenite::connect_async(&url).await?;
        info!("Connected to {url}, status {:#?}", response.status());
        let (mut write, reader) = ws_stream.split();
        write.send(Text(self.ws_auth_msg())).await?;
        write.send(Text(settings.to_request(2))).await?;
        for (index, request) in requests.iter().enumerate() {
            let string_request = request.subscribe(index + 3);
            println!("string_request: {:#?}", string_request);
            write.send(Text(string_request)).await?;
        }
        let timeout = tokio::time::sleep(std::time::Duration::from_secs(seconds_to_disconnect));
        tokio::pin!(timeout);
        tokio::select!(
            biased;
            _ = &mut timeout => {
                info!("timeout");
                return Ok(());
            }
            listen_result = tokio::spawn(keep_listening(reader,orderbooks_rwl,time_and_sales_rwl)) => {
                if let Err(e) = listen_result.unwrap() {
                    error!("Error in websocket {:#?}", e);
                    return Err(Error::ConnectionClosed);
                }
            },
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
}
