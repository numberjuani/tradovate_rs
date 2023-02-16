
use serde_json::{json, Value};
use serde::Serialize;
use serde::Deserialize;
use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize,Default)]
#[serde(rename_all = "camelCase")]
pub enum MarketData {
    #[serde(rename(deserialize = "md"))]
    DepthOfMarket,
    #[default]
    Quote,
    Histogram,
    Chart,
    #[serde(rename(deserialize = "shutdown"))]
    Shutdown,
    Clock,
}


#[derive(Debug, Clone, PartialEq, PartialOrd,Default)]
pub struct MarketDataRequest {
    pub data_type: MarketData,
    pub symbol: String,
    pub contract_id: i64,
    pub historical_id: i64,
    pub start_date:Option<DateTime<Utc>>,
}
impl MarketDataRequest {
    pub fn new(data_type: MarketData, symbol: &str) -> Self {
        Self {
            data_type,
            symbol: symbol.to_string(),
            contract_id: 0,
            historical_id: 0,
            ..Default::default()
        }
    }
    pub fn historical_chart(data_type: MarketData, symbol: &str,date:DateTime<Utc>) -> Self {
        Self {
            data_type,
            symbol: symbol.to_string(),
            contract_id: 0,
            historical_id: 0,
            start_date: Some(date),
        }
    }
    pub fn subscribe(&self, request_id: usize) -> String {
        use MarketData::*;
        let endpoint = match self.data_type {
            DepthOfMarket => "md/subscribeDOM",
            Quote => "md/subscribeQuote",
            Histogram => "md/subscribeHistogram",
            Chart => "md/getChart",
            Shutdown => "shutdown",
            Clock => todo!(),
        };
        let request_body = if self.data_type != Chart {
            json!({
                "symbol": self.symbol
            })
        } else {
            self.get_tick_chart_request_body()
        };
        format!("{}\n{}\n\n{}", endpoint, request_id, request_body)
    }
    pub fn unsubscribe(&self, request_id: i32) -> String {
        let endpoint = match self.data_type {
            MarketData::DepthOfMarket => "md/unsubscribeDOM",
            MarketData::Quote => "md/unsubscribeQuote",
            MarketData::Histogram => "md/unsubscribeHistogram",
            MarketData::Chart => "md/cancelChart",
            MarketData::Shutdown => todo!(),
            MarketData::Clock => todo!(),
        };
        let request_body = if self.data_type != MarketData::Chart {
            json!({
                "symbol": self.symbol
            })
        } else {
            json!({
                "subscriptionId": self.historical_id
            })
        };
        format!("{}\n{}\n\n{}", endpoint, request_id, request_body)
    }
    pub fn summarize(&self) -> String {
        format!("{} {:?}", self.symbol, self.data_type)
    }
    pub fn get_tick_chart_request_body(&self) -> Value {
        let time_stamp = if let Some(date) = self.start_date {
            date
        } else {
            Utc::now()
        };
        let formatted = time_stamp.to_rfc3339();
        json!({
          "symbol": self.symbol,
          "chartDescription": {
            "underlyingType": "Tick",
            "elementSize": 1,
            "elementSizeUnit": "UnderlyingUnits"
          },
          "timeRange": {
              "asFarAsTimestamp": formatted,
          }
        })
    }
}



