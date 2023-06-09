use reqwest::Response;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::request::Request;
use crate::response::ResponseErrorState;

use super::{
    super::constant::{URL_ACCOUNTS, URL_SERVER},
    super::response::{
        AccountsInfo,
        AccountsInfoSource
    },
    super::response::{
        ResponseError,
        ResponseErrorBody,
        ResponseErrorSource
    },
};

impl AccountsInfo {
    pub async fn get_account_info() -> Result<Vec<Self>, ResponseError> {
        let res = Self::request().await?;
        let res_serialized = res.text().await.unwrap();
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(|e: ResponseErrorSource| {
                    ResponseError {
                        state: ResponseErrorState::from(e.error.name.as_str()),
                        error: ResponseErrorBody {
                            name: e.error.name,
                            message: e.error.message
                        },
                    }
                }).ok().unwrap())
        }

        serde_json::from_str(&res_serialized)
            .map(|x: Vec<AccountsInfoSource>| {
                x
                    .into_iter()
                    .map(|x| Self {
                        currency: x.currency(),
                        balance: x.balance(),
                        locked: x.locked(),
                        avg_buy_price: x.avg_buy_price(),
                        avg_buy_price_modified: x.avg_buy_price_modified(),
                        unit_currency: x.unit_currency()
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalJsonParseError,
                    error: ResponseErrorBody {
                        name: "internal_json_parse_error".to_owned(),
                        message: x.to_string()
                    },
                }
            })
    }

    async fn request() -> Result<Response, ResponseError> {
        let token_string = Self::set_token()?;
        
        reqwest::Client::new()
            .get(format!("{URL_SERVER}{URL_ACCOUNTS}"))
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(|x| {
                ResponseError {
                    state: ResponseErrorState::InternalReqwestError,
                    error: ResponseErrorBody {
                        name: "internal_reqwest_error".to_owned(),
                        message: x.to_string()
                    },
                }
            })
    }
}
