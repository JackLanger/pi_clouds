use std::collections::HashMap;

use super::ForecastRoot;
use reqwest::{self};

pub(crate) async fn get_data() -> Result<ForecastRoot, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut url = String::from(super::BASE_URL);

    url.push_str(super::MANNHEIM_ID.to_string().as_str());

    let response = client.get(url).send().await;

    match response.is_ok() {
        true => {
            let body = response?.text().await?;
            let conversion_result = serde_json::from_str::<HashMap<String, ForecastRoot>>(&body);
            match conversion_result {
                Ok(map) => {
                    let target = super::MANNHEIM_ID.to_string();
                    let forecast = map.get(&target).unwrap();
                    Ok(forecast.clone())
                }
                Err(_) => panic!("Failed to parse JSON"),
            }
        }
        false => Err(reqwest::Error::from(response.unwrap_err())),
    }
}

#[cfg(test)]
mod controller_tests;
