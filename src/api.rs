use reqwest::Error;

pub async fn get_request() -> Result<String, Error> {
    let response = reqwest::get("https://api.eorc20.io/inscription?limit=500&offset=0").await?;
    let body = response.text().await?;
    // let body = "{\"data\":[{\"id\":\"0x2dc3a78b398d4a6f967bd145abe10983f8cdac0669a57a36705e23f586fa7c22\",\"from\":\"0x58ad8b5702a695ba4e4f0e6b8ba47728cc56c32c\",\"to\":\"0x18ffd9a0c916344fe39696f4e963a81bcd94a168\",\"p\":\"eorc20\",\"op\":\"transfer\",\"tick\":\"eoss\",\"amt\":\"5630000\",\"block_number\":26108241,\"timestamp\":\"2024-02-01 06:35:29\"},{\"id\":\"0x02cad1d22923653b2065065f2812f35dfe48577ee150ce2738c48a48e6d7078b\",\"from\":\"0xaf88dda37e92b1136c77886db96a7693025421ca\",\"to\":\"0x18ffd9a0c916344fe39696f4e963a81bcd94a168\",\"p\":\"eorc20\",\"op\":\"transfer\",\"tick\":\"eoss\",\"amt\":\"3720000\",\"block_number\":26108238,\"timestamp\":\"2024-02-01 06:35:26\"},{\"id\":\"0xc60a1d3844baf88611973343f83d2cc1ae3d0991c5f7efcb3d5a107b8a11ebd4\",\"from\":\"0x778a87e9e1fdc8d28cc54a45b084d3760dccf2af\",\"to\":\"0x18ffd9a0c916344fe39696f4e963a81bcd94a168\",\"p\":\"eorc20\",\"op\":\"transfer\",\"tick\":\"eoss\",\"amt\":\"5340000\",\"block_number\":26108236,\"timestamp\":\"2024-02-01 06:35:24\"}";
    println!("body = {:?}", body);
    Ok(body.to_string())
}

//https://api.eorc20.io/inscription?limit=500&offset=0

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_get_request() {
        let result = get_request().await;
        assert!(result.is_ok());
    }
}