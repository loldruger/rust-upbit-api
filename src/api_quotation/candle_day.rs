use crate::response::ResponseError;

use super::super::constant::URL_SERVER;
use super::UrlAssociates;

use reqwest::header::ACCEPT;
use reqwest::{Response, Url};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CandleChartDay {
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
    prev_closing_price: f64,
    change_price: f64,
    change_rate: f64,
    converted_trade_price: Option<f64>,
}

impl CandleChartDay {
    pub async fn request_candle(
        market: &str,
        count: i32,
        last_candle_time: Option<String>,
        price_unit: Option<String>,
    ) -> Result<Vec<Self>, ResponseError> {
        let res = Self::request(market, count, last_candle_time, price_unit).await?;
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)?;

        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)
                .ok()
                .unwrap());
        }

        serde_json::from_str(&res_serialized)
            .map(|i: Vec<Self>| {
                i.into_iter()
                    .map(|x| Self {
                        market: x.market,
                        candle_date_time_utc: x.candle_date_time_utc,
                        candle_date_time_kst: x.candle_date_time_kst,
                        opening_price: x.opening_price,
                        high_price: x.high_price,
                        low_price: x.low_price,
                        trade_price: x.trade_price,
                        timestamp: x.timestamp,
                        candle_acc_trade_price: x.candle_acc_trade_price,
                        candle_acc_trade_volume: x.candle_acc_trade_volume,
                        prev_closing_price: x.prev_closing_price,
                        change_price: x.change_price,
                        change_rate: x.change_rate,
                        converted_trade_price: x.converted_trade_price,
                    })
                    .collect()
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request(
        market: &str,
        count: i32,
        last_candle_time: Option<String>,
        price_unit: Option<String>,
    ) -> Result<Response, ResponseError> {
        let url_candle = UrlAssociates::UrlCandleDay.to_string();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}"))
            .map_err(crate::response::response_error_internal_url_parse_error)?;

        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str());

        if let Some(last_candle_time) = last_candle_time {
            url.query_pairs_mut()
                .append_pair("to", last_candle_time.as_str());
        }

        if let Some(price_unit) = price_unit {
            url.query_pairs_mut()
                .append_pair("convertingPriceUnit", price_unit.as_str());
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

    use crate::api_quotation::CandleChartDay;

    #[tokio::test]
    async fn test_request_candle_day() {
        crate::set_access_key(&std::env::var("TEST_ACCESS_KEY").expect("TEST_ACCESS_KEY not set"));
        crate::set_secret_key(&std::env::var("TEST_SECRET_KEY").expect("TEST_SECRET_KEY not set"));

        let res = CandleChartDay::request("KRW-ETH", 1, None, None)
            .await
            .unwrap();
        let res_serialized = res
            .text()
            .await
            .map_err(crate::response::response_error_from_reqwest)
            .unwrap();

        if res_serialized.contains("error") {
            assert!(false, "Error response: {res_serialized}");
        }

        let json = serde_json::from_str::<Value>(&res_serialized)
            .map_err(crate::response::response_error_from_json)
            .unwrap();
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
            "prev_closing_price": "",
            "change_price": "",
            "change_rate": "",
            // "converted_trade_price": ""
        }]);

        let expected_structure = expected_structure[0]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<HashMap<&str, Value>>();

        if let Some(json_array) = json.as_array() {
            for (index, item) in json_array.iter().enumerate() {
                let (missing_keys, extra_keys) =
                    compare_keys(item, &expected_structure, &format!("item[{}].", index));

                if !missing_keys.is_empty() {
                    println!(
                        "[test_request_candle_day] Missing keys in item[{}]: {:?}",
                        index, missing_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_request_candle_day] No keys are missing in item[{}]",
                        index
                    );
                    assert!(true);
                }

                if !extra_keys.is_empty() {
                    println!(
                        "[test_request_candle_day] Extra keys in item[{}]: {:?}",
                        index, extra_keys
                    );
                    assert!(false);
                } else {
                    println!(
                        "[test_request_candle_day] No extra keys found in item[{}]",
                        index
                    );
                    assert!(true);
                }
            }
        } else {
            assert!(false, "Expected an array of objects in the response");
        }
    }

    fn compare_keys(
        json: &Value,
        expected: &HashMap<&str, Value>,
        path: &str,
    ) -> (Vec<String>, Vec<String>) {
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
