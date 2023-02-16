use chrono::Utc;
use futures::StreamExt;
use log::error;
use log::info;
use log::warn;
use serde::Serialize;
use serde::Deserialize;
use chrono::DateTime;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message;


use crate::models::orderbook::OrderBooksRWL;
use crate::models::quotes::QuotesRWL;
use crate::models::time_and_sales::TimeAndSalesRWL;

use super::connection::ReadWs;
use super::process_replay_ms::parse_replay_messages;


#[derive(Serialize, Deserialize, Debug,Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MarketReplaySettings {
    #[serde(serialize_with = "parse_mr_date")]
    pub start_timestamp: DateTime<Utc>,
    pub speed: i64,
    pub initial_balance: i64,
}
impl MarketReplaySettings {
    pub fn to_request(&self,request_num:i64) -> String {
        format!("replay/initializeclock\n{}\n\n{}",request_num,serde_json::to_string(self).unwrap())
    }
}


//"%Y-%m-%dT%H:%M:%S%.fZ"
pub fn parse_mr_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = format!("{}", date.format("%Y-%m-%dT%H:%M:%S%.3f%.fZ"));
    serializer.serialize_str(&s)
}

pub async fn replay_messages(
    mut reader:ReadWs,
    orderbooks_rwl: OrderBooksRWL,
    time_and_sales_rwl: TimeAndSalesRWL,
    quotes: QuotesRWL,
    end_time:DateTime<Utc>,
) -> Result<(), Error> {
    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => {
                match msg {
                    Message::Text(txtmsg) => {
                        match parse_replay_messages(txtmsg, orderbooks_rwl.clone(), time_and_sales_rwl.clone(),quotes.clone(),end_time).await {
                            Ok(success) => {
                                if success {
                                    info!("Job complete");
                                    return Ok(());
                                }
                            },
                            Err(e) => {
                                error!("Error parsing replay message: {:#?}", e);
                                return Err(Error::ConnectionClosed);
                            }
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