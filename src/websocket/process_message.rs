use serde_json::{Value, Map};


use crate::models::{tick_chart::ChartData, orderbook::OrderBooks};
use log::error;
use super::requests::MarketData;


pub fn parse_messages(message:String) {
    if message.len() < 3 {
        return;
    }
    match serde_json::from_str::<Map<String,Value>>(&message[2..message.len()-1]) {
        Ok(json_data) => {
            if json_data.contains_key("e") {
                match serde_json::from_str::<MarketData>(&json_data["e"].to_string()) {
                    Ok(data_type) => {
                        match data_type {
                            MarketData::DepthOfMarket => {
                                match serde_json::from_value::<OrderBooks>(json_data["d"].clone()) {
                                    Ok(dom_data) => {
                                        println!("{:#?}", dom_data);
                                    },
                                    Err(e) => error!("error parsing dom data: {}", e)
                                }
                            },
                            MarketData::Quote => todo!(),
                            MarketData::Histogram => todo!(),
                            MarketData::Chart => {
                                match serde_json::from_value::<ChartData>(json_data["d"].clone()) {
                                    Ok(chart_data) => {
                                        println!("{:#?}", chart_data.get_all_ts_items());
                                    },
                                    Err(e) => error!("error parsing chart data: {}", e)
                                }
                            },
                        }
                    },
                    Err(e) => error!("error parsing market data type: {}", e)
                }
            }
        },
        Err(e) => error!("error parsing message: {}", e)
    }
}