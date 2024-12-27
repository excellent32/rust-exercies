use reqwest::Error;
use serde::Deserialize;
use serde_json::{json};
use std::fmt;

#[derive(Deserialize)]
struct Token {
    address: String,
    chainId: u32,
    symbol: String,
    decimals: u8,
    name: String,
    coinKey: String,
    logoURI: String,
    priceUSD: String
}

#[derive(Deserialize)]
struct TokenKey {
    token: Token,
    amount: String,
    amountUsd: String
}

#[derive(Deserialize)]
pub struct Suggestion {
    recommended: TokenKey,
    limit: TokenKey,
    serviceFee: TokenKey,
    available: bool,
    fromAmount: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let ele: Vec<Prices> = self.0.iter().map(|s| s.to_string()).collect();
        write!(f, "chainId: {}, symbol: {}", self.chainId, self.symbol)
    }
}

impl fmt::Display for TokenKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "token: {}, amount: {}, amountUsd: {}", self.token, self.amount, self.amountUsd)
    }
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "recommended: {}, limit: {}, serviceFee: {}, available: {}, fromAmount: {}", self.recommended, self.limit, self.serviceFee, self.available, self.fromAmount)
    }
}

static GAS_URL: &str = "https://li.quest/v1/gas/suggestion/10";

pub async fn get_gas_suggestion() -> Result<Suggestion, Error>{
    let reponse = reqwest::get(GAS_URL)
    .await?.json::<Suggestion>().await?;
    // print!("Response: {:?}", &reponse);

    Ok(reponse)
}