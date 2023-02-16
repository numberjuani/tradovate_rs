use chrono::{DateTime, Utc};
use log::{error, warn, info};
use serde_json::{Value, Map};

use crate::{models::{orderbook::{OrderBooksRWL, OrderBooks}, time_and_sales::TimeAndSalesRWL, tick_chart::ChartData, quotes::{Quotes, QuotesRWL}, replay_clock::ReplayClock}, websocket::process_message::TradovateWSError};

use super::requests::MarketData;


///Returns true if the job is complete. It is configured mostly to use market replay to gather data.
pub async fn parse_replay_messages(message:String,orderbooks_rwl:OrderBooksRWL,time_and_sales_rwl:TimeAndSalesRWL,quotes:QuotesRWL,end_time:DateTime<Utc>) -> Result<bool,TradovateWSError> {
    if message.len() < 3 {
        return Ok(false)
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
                                        Ok(false)
                                    },
                                    Err(e) => {
                                        error!("error parsing dom data: {}", e);
                                        Err(TradovateWSError::ParseError(e))
                                    }
                                }
                            },
                            MarketData::Quotes => {
                                match serde_json::from_value::<Quotes>(json_data["d"].clone()) {
                                    Ok(quote) => {
                                        let mut quotes = quotes.write().await;
                                        quotes.push(quote);
                                        Ok(false)
                                    },
                                    Err(e) => {
                                        error!("error parsing dom data: {}", e);
                                        Err(TradovateWSError::ParseError(e))
                                    }
                                }
                            },
                            MarketData::Histogram => todo!(),
                            MarketData::Chart => {
                                match serde_json::from_value::<ChartData>(json_data["d"].clone()) {
                                    Ok(chart_data) => {
                                        let ts_items = chart_data.get_all_ts_items();
                                        let mut ts = time_and_sales_rwl.write().await;
                                        ts.append(&mut ts_items.clone());
                                        Ok(false)
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
                                match serde_json::from_str::<ReplayClock>(json_data["d"].clone().as_str().unwrap()) {
                                    Ok(p_clock) => {
                                        println!("{:#?}",p_clock);
                                        return Ok(p_clock.time >= end_time)
                                    },
                                    Err(e) => {
                                        error!("error parsing clock data: {}", e);
                                        Err(TradovateWSError::ParseError(e))
                                    }
                                }
                            },
                        }
                    },
                    Err(e) => {
                        error!("error parsing market data type: {}", e);
                        Ok(false)
                    }
                }
            } else if json_data.contains_key("s") {
                if json_data["s"].as_i64().unwrap() == 200 {
                    info!("successfully subscribed to market data");
                    return Ok(false)
                } else {
                    error!("received error message from server");
                    warn!("{}", message);
                    return Err(TradovateWSError::UnknownError(message))
                }
            } else {
                error!("received unknown message from server");
                warn!("{}", message);
                return Ok(false)
            }
        },
        Err(e) => {
            error!("error parsing message: {}", e);
            Err(TradovateWSError::ParseError(e))
        }
    }
}