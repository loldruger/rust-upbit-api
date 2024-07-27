use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Url, Response};

use crate::constant::TwoFactorType;

use super::super::{
        constant::{URL_WITHDRAWS_KRW, URL_SERVER},
        request::RequestWithQuery,
        response::{
            TransactionInfo,
            TransactionInfoSource,
            ResponseError
        }
    };

impl TransactionInfo {
    pub async fn withdraw_krw(amount: f64, two_factor_type: TwoFactorType) -> Result<Self, ResponseError> {
        let res = Self::request_withdraw_krw(amount, two_factor_type).await?;
        let res_serialized = res.text().await.unwrap();
        
        if res_serialized.contains("error") {
            return Err(serde_json::from_str(&res_serialized)
                .map(crate::response::response_error)                
                .ok()
                .unwrap()
            )
        }

        serde_json::from_str(&res_serialized)
            .map(|x: TransactionInfoSource| {
                Self {
                    r#type: x.r#type(),
                    uuid: x.uuid(),
                    currency: x.currency(),
                    net_type: x.net_type(),
                    txid: x.txid(),
                    state: x.state(),
                    created_at: x.created_at(),
                    done_at: x.done_at(),
                    amount: x.amount(),
                    fee: x.fee(),
                    transaction_type: x.transaction_type(),
                }
            })
            .map_err(crate::response::response_error_from_json)
    }

    async fn request_withdraw_krw(amount: f64, two_factor_type: TwoFactorType) -> Result<Response, ResponseError> {
        let mut url = Url::parse(&format!("{URL_SERVER}{URL_WITHDRAWS_KRW}")).unwrap();
        
        url.query_pairs_mut()
            .append_pair("amount", &format!("{amount}"))
            .append_pair("two_factor_type", &two_factor_type.to_string());
            
        let token_string = Self::set_token_with_query(url.as_str())?;
        
        reqwest::Client::new()
            .post(url.as_str())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &token_string)
            .send()
            .await
            .map_err(crate::response::response_error_from_reqwest)
    }
}
