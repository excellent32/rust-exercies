use reqwest::Error;
use serde::Deserialize;
use serde_json::{json};
use std::fmt;
use std::collections::HashMap;
 

static GAS_URL: &str = "https://li.quest/v1/gas/prices";

#[derive(Deserialize, Debug)]
pub struct Price {
    standard: u64,
    fast: u64,
    fastest: u64,
    lastUpdated: u64
}

impl Price {
    
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let ele: Vec<Prices> = self.0.iter().map(|s| s.to_string()).collect();
        write!(f, "standard: {}, fast: {}, fastest: {}, lastUpdated: {}", self.standard, self.fast, self.fastest, self.lastUpdated)
    }
}

pub async fn get_gas_fee() -> Result<HashMap<String, Price>, Error>{
    let reponse = reqwest::get(GAS_URL)
    .await?.json::<HashMap<String, Price>>().await?;
    // print!("Response: {:?}", &reponse);

    Ok(reponse)
}
