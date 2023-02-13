

use crate::client::{Protocol, ResourceType, TradovateClient};
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
use super::requests::MarketDataRequest;
use crate::websocket::connection::Message::Text;

pub async fn keep_listening(
    mut reader:ReadWs,
    on_message: fn(String) -> (),
    max_num_messages: Option<i64>,
) -> Result<(), Error> {
    let mut message_num = 0;
    while let Some(msg) = reader.next().await {
        on_message(msg?.into_text()?);
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
        on_message: fn(String) -> (),
        requests: Vec<MarketDataRequest>,
        max_num_messages: Option<i64>,
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
            _ = tokio::spawn(keep_listening(reader, on_message,max_num_messages)) => {},
            _ = tokio::spawn(send_heartbeats(write)) => {}
        );
        Ok(())
    }
}
