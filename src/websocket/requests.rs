
use serde_json::{json, Value};
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketData {
    #[serde(rename(deserialize = "md"))]
    DepthOfMarket,
    Quote,
    Histogram,
    Chart,
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MarketDataRequest {
    pub data_type: MarketData,
    pub symbol: String,
    pub contract_id: i64,
    pub historical_id: i64,
}
impl MarketDataRequest {
    pub fn new(data_type: MarketData, symbol: &str) -> Self {
        Self {
            data_type,
            symbol: symbol.to_string(),
            contract_id: 0,
            historical_id: 0,
        }
    }
    pub fn subscribe(&self, request_id: usize) -> String {
        use MarketData::*;
        let endpoint = match self.data_type {
            DepthOfMarket => "md/subscribeDOM",
            Quote => "md/subscribeQuote",
            Histogram => "md/subscribeHistogram",
            Chart => "md/getChart",
        };
        let request_body = if self.data_type != Chart {
            json!({
                "symbol": self.symbol
            })
        } else {
            get_tick_chart_request_body(&self.symbol)
        };
        format!("{}\n{}\n\n{}", endpoint, request_id, request_body)
    }
    pub fn unsubscribe(&self, request_id: i32) -> String {
        let endpoint = match self.data_type {
            MarketData::DepthOfMarket => "md/unsubscribeDOM",
            MarketData::Quote => "md/unsubscribeQuote",
            MarketData::Histogram => "md/unsubscribeHistogram",
            MarketData::Chart => "md/cancelChart",
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
}

pub fn get_tick_chart_request_body(symbol: &str) -> Value {
    let time_stamp = chrono::Utc::now();
    let formatted = time_stamp.to_rfc3339();
    json!({
      "symbol": symbol,
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