

use crate::{client::{Protocol, ResourceType, TradovateClient}, models::{orderbook::OrderBooksRWL, time_and_sales::TimeAndSalesRWL}};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{net::TcpStream};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
pub type WriteWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type ReadWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
use super::{requests::MarketDataRequest, process_message::parse_messages};
use crate::websocket::connection::Message::Text;

pub async fn keep_listening(
    mut reader:ReadWs,
    max_num_messages: Option<i64>,
    orderbooks_rwl: OrderBooksRWL,
    time_and_sales_rwl: TimeAndSalesRWL,
) -> Result<(), Error> {
    let mut message_num = 0;
    while let Some(msg) = reader.next().await {
        parse_messages(msg?.into_text()?, orderbooks_rwl.clone(), time_and_sales_rwl.clone()).await;
        message_num += 1;
        if let Some(max_num_messages) = max_num_messages {
            if message_num >= max_num_messages {
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
        requests: Vec<MarketDataRequest>,
        max_num_messages: Option<i64>,
        orderbooks_rwl: OrderBooksRWL,
        time_and_sales_rwl: TimeAndSalesRWL,
    ) -> Result<(), Error> {
        let url = self.url(ResourceType::MarketData, Protocol::Wss);
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut write, reader) = ws_stream.split();
        write.send(Text(self.ws_auth_msg())).await?;
        for (index, request) in requests.iter().enumerate() {
            write.send(Text(request.subscribe(index + 2))).await?;
        }
        tokio::select!(
            biased;
            _ = tokio::spawn(keep_listening(reader,max_num_messages,orderbooks_rwl,time_and_sales_rwl)) => {},
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
}
