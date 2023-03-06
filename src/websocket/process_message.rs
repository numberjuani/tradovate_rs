use std::sync::Arc;

use serde_json::{Value, Map};
use tokio::sync::Notify;


use crate::models::{tick_chart::ChartData, orderbook::{OrderBooks, OrderBooksRWL}, time_and_sales::TimeAndSalesRWL};
use log::{error, warn, info};
use super::requests::MarketData;

#[derive(Debug)]
pub enum TradovateWSError {
    ConnectionError,
    ParseError(serde_json::Error),
    UnknownError(String),
    TooManyRetries,
}

pub async fn parse_messages(message:String,orderbooks_rwl:OrderBooksRWL,time_and_sales_rwl:TimeAndSalesRWL,notify:Arc<Notify>,threshold:usize) -> Result<(),TradovateWSError> {
    if message.len() < 3 {
        return Ok(())
    }
    match serde_json::from_str::<Map<String,Value>>(&message[2..message.len()-1]) {
        Ok(json_data) => {
            if json_data.contains_key("e") {
                let key_to_match = if json_data["e"].as_str().unwrap() == "md" {
                    json_data["d"].clone().as_object().unwrap().keys().next().unwrap().to_string()
                } else {
                    json_data["e"].clone().as_str().unwrap().to_string()
                };
                match MarketData::from_string(&key_to_match) {
                    Ok(data_type) => {
                        match data_type {
                            MarketData::DepthOfMarket => {
                                match serde_json::from_value::<OrderBooks>(json_data["d"].clone()) {
                                    Ok(dom_data) => {
                                        let mut books = orderbooks_rwl.write().await;
                                        books.push(dom_data);
                                        Ok(())
                                    },
                                    Err(e) => {
                                        error!("error parsing dom data: {}", e);
                                        Err(TradovateWSError::ParseError(e))
                                    }
                                }
                            },
                            MarketData::Quotes => {
                                Ok(())
                            },
                            MarketData::Histogram => todo!(),
                            MarketData::Chart => {
                                match serde_json::from_value::<ChartData>(json_data["d"].clone()) {
                                    Ok(chart_data) => {
                                        let ts_items = chart_data.get_all_ts_items();
                                        let mut ts = time_and_sales_rwl.write().await;
                                        ts.append(&mut ts_items.clone());
                                        if ts.len() > threshold {
                                            notify.notify_one();
                                        }
                                        Ok(())
                                    },
                                    Err(e) => {
                                        error!("error parsing chart data: {}", e);
                                        Err(TradovateWSError::ParseError(e))
                                    }
                                }
                            },
                            MarketData::Shutdown => {
                                error!("received shutdown message from server");
                                warn!("{}", message);
                                return Err(TradovateWSError::ConnectionError)
                            },
                            MarketData::Clock => {
                                info!("received clock message from server");
                                return Ok(())
                            },
                        }
                    },
                    Err(e) => {
                        error!("error parsing market data type: {}", e);
                        Err(TradovateWSError::ParseError(e))
                    }
                }
            } else if json_data.contains_key("s") {
                if json_data["s"].as_i64().unwrap() == 200 {
                    info!("successfully subscribed to market data");
                    return Ok(())
                } else {
                    error!("received error message from server");
                    warn!("{}", message);
                    return Err(TradovateWSError::UnknownError(message))
                }
            } else {
                error!("received unknown message from server");
                warn!("{}", message);
                return Ok(())
            }
        },
        Err(e) => {
            error!("error parsing message: {}", e);
            Err(TradovateWSError::ParseError(e))
        }
    }
}

