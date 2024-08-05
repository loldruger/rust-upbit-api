use crate::response::ResponseError;

use super::UrlAssociates;
use super::super::constant::URL_SERVER;

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CandleChartMonth {
    market: String,
    candle_date_time_utc: String,
    candle_date_time_kst: String,
    opening_price: f64,
    high_price: f64,
    low_price: f64,
    trade_price: f64,
    timestamp: i64,
    candle_acc_trade_price: f64,
    candle_acc_trade_volume: f64,
    first_day_of_period: String
}

impl CandleChartMonth {
    pub async fn request_candle(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(market, count, last_candle_time).await?;
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest)?;
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }
        
        serde_json::from_str(&res_serialized)
            .map(|x: Vec<Self>| {
                x
                    .into_iter()
                    .map(|i| {
                        Self {
                            market: i.market,
                            candle_date_time_utc: i.candle_date_time_utc,
                            candle_date_time_kst: i.candle_date_time_kst,
                            opening_price: i.opening_price,
                            high_price: i.high_price,
                            low_price: i.low_price,
                            trade_price: i.trade_price,
                            timestamp: i.timestamp,
                            candle_acc_trade_price: i.candle_acc_trade_price,
                            candle_acc_trade_volume: i.candle_acc_trade_volume,
                            first_day_of_period: i.first_day_of_period,
                        }
                    })
                    .collect()
        })
        .map_err(crate::response::response_error_from_json)
    }

    async fn request(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Response, ResponseError> {
        let url_candle = UrlAssociates::UrlCandleMonth.to_string();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str());

        if let Some(last_candle_time) = last_candle_time {
            url.query_pairs_mut().append_pair("to", last_candle_time.as_str());
        }

        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::api_quotation::CandleChartMonth;

    #[tokio::test]
    async fn test_request_candle() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = CandleChartMonth::request("KRW-ETH", 1, None).await.unwrap();
        let res_serialized = res.text().await.map_err(crate::response::response_error_from_reqwest).unwrap();
        
        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized).map_err(crate::response::response_error_from_json).unwrap();
        let expected_structure = serde_json::json!([{
            "market": "",
            "candle_date_time_utc": "",
            "candle_date_time_kst": "",
            "opening_price": "",
            "high_price": "",
            "low_price": "",
            "trade_price": "",
            "timestamp": "",
            "candle_acc_trade_price": "",
            "candle_acc_trade_volume": "",
            "first_day_of_period": ""
        }]);

        let expected_structure = expected_structure[0]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        if let Some(json_array) = json.as_array() {
            for (index, item) in json_array.iter().enumerate() {
                let (missing_keys, extra_keys) = compare_keys(item, &expected_structure, &format!("item[{}].", index));
    
                if !missing_keys.is_empty() {
                    println!("[test_get_order_state_list] Missing keys in item[{}]: {:?}", index, missing_keys);
                    assert!(false);
                } else {
                    println!("[test_get_order_state_list] No keys are missing in item[{}]", index);
                    assert!(true);
                }
    
                if !extra_keys.is_empty() {
                    println!("[test_get_order_state_list] Extra keys in item[{}]: {:?}", index, extra_keys);
                    assert!(false);
                } else {
                    println!("[test_get_order_state_list] No extra keys found in item[{}]", index);
                    assert!(true);
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }
    }

    fn compare_keys(json: &Value, expected: &HashMap<&str, Value>, path: &str) -> (Vec<String>, Vec<String>) {
        let mut missing_keys = Vec::new();
        let mut extra_keys = Vec::new();
    
        if let Some(actual_map) = json.as_object() {
            for (key, _) in expected {
                if !actual_map.contains_key(*key) {
                    missing_keys.push(format!("{}{}", path, key));
                }
            }
            for (key, _) in actual_map {
                if !expected.contains_key(key.as_str()) {
                    extra_keys.push(format!("{}{}", path, key));
                }
            }
        }
    
        (missing_keys, extra_keys)
    }
}