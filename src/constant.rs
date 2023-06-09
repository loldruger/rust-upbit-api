/// Server domain address
pub const URL_SERVER: &str = "https://api.upbit.com";

/// URL of API getting account info
pub const URL_ACCOUNTS: &str = "/v1/accounts";

/// URL of API getting order info  
pub const URL_ORDER: &str = "/v1/orders";
/// URL of API getting order chance
pub const URL_ORDER_CHANCE: &str = "/v1/orders/chance";
/// URL of API getting order status
pub const URL_ORDER_STATUS: &str = "/v1/order";
/// URL of API getting order status list
pub const URL_ORDER_STATUS_LIST: &str = "/v1/orders";

/// URL of API getting order book
pub const URL_ORDERBOOK: &str = "/v1/orderbook";
/// URL of API getting ticker
pub const URL_TICKER: &str = "/v1/ticker";
/// URL of API getting trandes ticks
pub const URL_TRADES_TICKS: &str = "/v1/trades/ticks";
/// URL of API getting market state
pub const URL_MARKET_STATE: &str = "/v1/market/all";

/// URL of API getting withdraw info
pub const URL_WITHDRAW: &str = "/v1/withdraw";
/// URL of API getting withdraw info list
pub const URL_WITHDRAWS: &str = "/v1/withdraws";
/// URL of API withdrawing KRW
pub const URL_WITHDRAWS_KRW: &str = "/v1/withdraws/krw";
/// URL of API withdrawing coin
pub const URL_WITHDRAWS_COIN: &str = "/v1/withdraws/coin";
/// URL of API getting coin address
pub const URL_WITHDRAWS_COIN_ADDRESS: &str = "v1/withdraws/coin_addresses";
/// URL of API getting withdraw chance
pub const URL_WITHDRAWS_CHANCE: &str = "/v1/withdraws/chance";

/// URL of deposit API
pub const URL_DEPOSIT: &str = "/v1/deposit";
/// URL of API listing Deposit info
pub const URL_DEPOSITS: &str = "/v1/deposits";
/// URL of API inquiring generation of coin deposit address
pub const URL_DEPOSITS_GENERATE_COIN_ADDRESS: &str = "/v1/deposits/generate_coin_address";
/// URL of API getting asset you have
pub const URL_DEPOSITS_COIN_ADDRESS: &str = "/v1/deposits/coin_address";
/// URL of API listing assets you have
pub const URL_DEPOSITS_COIN_ADDRESSES: &str = "/v1/deposits/coin_addresses";
/// URL of API requesting to deposit KRW
pub const URL_DEPOSITS_KRW: &str = "/v1/deposits/krw";

/// URL of API listing candle data of minute unit
pub const URL_CANDLE_MINUTE: &str = "/v1/candles/minutes/";
/// URL of API listing candle data of day unit
pub const URL_CANDLE_DAY: &str = "/v1/candles/days";
/// URL of API listing candle data of week unit
pub const URL_CANDLE_WEEK: &str = "/v1/candles/weeks";
/// URL of API listing candle data of month unit
pub const URL_CANDLE_MONTH: &str = "/v1/candles/months";

/// Kind of order 
pub enum OrderBy {
    /// 오름차순 (Ascending)
    Asc,
    /// 내림차순 (Descending)
    Desc
}

impl ToString for OrderBy {
    fn to_string(&self) -> String {
        match self {
            OrderBy::Asc => "asc".to_owned(),
            OrderBy::Desc => "desc".to_owned(),
        }
    }
}

impl From<&str> for OrderBy {
    fn from(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            _ => panic!()
        }
    }
}

/// Kind of transaction type
#[derive(Debug)]
pub enum WithdrawType {
    /// 일반 입출금(general withdrawal or deposit)
    Default,
    /// 바로 입출금(instant withdrawal or deposit)
    Internal 
}

impl ToString for WithdrawType {
    fn to_string(&self) -> String {
        match self {
            WithdrawType::Default => "default".to_owned(),
            WithdrawType::Internal => "internal".to_owned(),
        }
    }
}

impl From<&str> for WithdrawType {
    fn from(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "internal" => Self::Internal,
            _ => panic!("")
        }
    }
}

/// Kind of tow factor type
pub enum TwoFactorType {
    /// 카카오페이 인증
    KakaoPay,
    /// 네이버 인증
    Naver
}

impl ToString for TwoFactorType {
    fn to_string(&self) -> String {
        match self {
            TwoFactorType::KakaoPay => "kakao_pay".to_owned(),
            TwoFactorType::Naver => "naver".to_owned(),
        }
    }
}

/// List of transaction type
#[derive(Debug)]
pub enum TransactionType {
    /// 출금
    Withdraw,
    /// 입금
    Deposit
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Withdraw => "withdraw".to_owned(),
            TransactionType::Deposit => "deposit".to_owned(),
        }
    }
}

impl From<&str> for TransactionType {
    fn from(value: &str) -> Self {
        match value {
            "withdraw" => Self::Withdraw,
            "deposit" => Self::Deposit,
            _ => panic!("")
        }
    }
}

// pub enum Currency {
//     KRW,
//     BTC,
//     USDT,
// }

// pub enum CurrencyCrypto {
//     BTC,
//     ETH,
//     ATOM
// }

// pub struct MarketType(Currency, CurrencyCrypto);
