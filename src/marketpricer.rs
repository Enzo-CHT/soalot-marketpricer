// Sandbox : https://sandbox-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest
// Pro : https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest

// Request : curl -H "X-CMC_PRO_API_KEY: <APIKEY>" -H "Accept: application/json" -d "convert=USD&symbol=BTC" -G https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest

use chrono::prelude::{DateTime, Utc};

#[derive(Debug)]
pub struct TokenRecord {
    token: Token,
    date: DateTime<Utc>,
    usd_price: f64,
}

impl TokenRecord {
    pub fn new(token: Token, usd_price: f64) -> Self {
        Self {
            token,
            usd_price,
            date: Utc::now(),
        }
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self.date
    }
    pub fn get_token(&self) -> &Token {
        &self.token
    }
    pub fn get_price(&self) -> f64 {
        self.usd_price
    }
}

#[derive(Debug)]
pub struct Token {
    pub id: i32,
    pub slug: String,
    pub symbol: String,
}

pub struct MarketPricer {
    api_key: String,
    uri: String,
}

impl MarketPricer {
    fn save_record(&self, record: TokenRecord) -> () {
        // blank
    }

    pub fn new(&self, uri: String) -> Self {
        Self {
            uri,
            api_key: String::new(),
        }
    }

    pub fn set_api_key(&mut self, api_key: String) -> () {
        self.api_key = api_key;
    }

    pub fn update_price(&self, token: Token) -> TokenRecord {
        TokenRecord::new(token, 0.0)
        // blank
    }
}
