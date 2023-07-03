use crate::response_source::{ResponseErrorSource, ResponseErrorBodySource};

use super::super::constant::{URL_SERVER, UrlAssociates};

use reqwest::{Url, Response};
use reqwest::header::ACCEPT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    pub async fn request_candle(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Vec<Self>, ResponseErrorSource> {
        let res = Self::request(market, count, last_candle_time).await?;
        let res_serialized = res.text().await.unwrap();
        
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
        .map_err(|error| {
            eprintln!("{}", error);
        
            serde_json::from_str(&res_serialized).unwrap()
        })
    }

    async fn request(market: &str, count: i32, last_candle_time: Option<String>) -> Result<Response, ResponseErrorSource> {
        let url_candle: String = UrlAssociates::UrlCandleMonth.into();
        let mut url = Url::parse(&format!("{URL_SERVER}{url_candle}")).unwrap();
        url.query_pairs_mut()
            .append_pair("market", market)
            .append_pair("count", count.to_string().as_str());

        if last_candle_time.is_some() {
            url.query_pairs_mut().append_pair("to", last_candle_time.unwrap().as_str());
        }
        
        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(|x| {
                ResponseErrorSource {
                    error: ResponseErrorBodySource {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    }
                }
            })
    }
}