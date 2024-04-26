use std::str;
use serde_json::Value;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BigInt;

pub fn json_to_i64(value: &serde_json::Value, index: &str) -> Option<i64> {
    if let Some(value) = value.get(index) {
        if let Some(str) = value.as_str() {
            let parsed = str.parse::<i64>();
            match parsed {
                Ok(parsed) => Some(parsed),
                Err(_e) => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn json_to_string(value: &serde_json::Value, index: &str) -> String {
    match value.get(index) {
        Some(value) => {
            match value.as_str() {
                Some(str) => str.to_string(),
                None => "".to_string(),
            }
        },
        None => "".to_string(),
    }
}

pub fn parse_value(value: &Option<BigInt>) -> String {
    match value {
        Some(big_int) => {
            if Hex(&big_int.bytes).to_string().len() == 0 {
                String::from("0")
            } else {
                Hex(&big_int.bytes).to_string()
            }
        },
        None => String::from("0")
    }
}

pub fn validate_data(input: String) -> Result<(String, String, String, Value), String> {
    let json_str = input.splitn(2, ',').nth(1).unwrap_or_default();
    let json_data = match serde_json::from_str(json_str) {
        Ok(data) => data,
        Err(_e) => return Err("An error occurred".into()),
    };

    let tick = json_to_string(&json_data, "tick");
    let op = json_to_string(&json_data, "op");
    let p = json_to_string(&json_data, "p");

    Ok((tick, op, p, json_data))
}

pub fn validate_utf8(data: &Vec<u8>) -> Result<bool, bool> {
    let input = match str::from_utf8(data) {
        Ok(_vec) => true,
        Err(_e) => false,
    };
    // ignore empty calldata
    Ok(input && !data.is_empty())
}

pub fn get_mime_type(input: &String) -> Result<(&'static str, &'static str, &'static str), String>{
    let media_type: &str;
    let mime_subtype: &str;
    let mime_type: &str;

    if input.starts_with("data:application/json") {
        media_type = "application";
        mime_subtype = "json";
        mime_type = "application/json";
        return Ok((media_type, mime_subtype, mime_type))
    }
    if input.starts_with("data") {
        media_type = "text/plain";
        mime_subtype = "plain";
        mime_type = "text/plain";
        return Ok((media_type, mime_subtype, mime_type))
    }
    if input.starts_with("data:text/plain") {
        media_type = "text";
        mime_subtype = "plain";
        mime_type = "text/plain";
        return Ok((media_type, mime_subtype, mime_type))
    }
    else {
        return Err("Invalid data type".into())
    }
}
#[cfg(test)]
mod tests {
    use substreams::Hex;
    use substreams_ethereum::pb::eth::v2::BigInt;
    #[test]
    fn json_to_string() {
        let str = "{
            \"op\": \"mint\",
            \"tick\": \"bull\"
        }";
        let data = serde_json::from_str(str);

        match data {
            Ok(data) => {
                assert_eq!(super::json_to_string(&data, "op"), "mint");
                assert_eq!(super::json_to_string(&data, "tick"), "bull");
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn json_to_i64() {
        let str = "{
            \"amt\": \"100\"
        }";
        let data = serde_json::from_str(str);

        match data {
            Ok(data) => {
                assert_eq!(super::json_to_i64(&data, "amt").unwrap(), 100);
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn max_json_to_i64() {
        let str = "{
            \"max\": \"2.1e+29\",
            \"lim\": \"100000000000000\"
        }";
        let data = serde_json::from_str(str);

        match data {
            Ok(data) => {
                assert_eq!(super::json_to_i64(&data, "max").is_none(), true);
                assert_eq!(super::json_to_i64(&data, "lim").unwrap(), 100000000000000);
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn parse_value() {
        let str = "{
            \"value\": \"0x646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d\"
        }";
        let data = serde_json::from_str::<serde_json::Value>(str);

        match data {
            Ok(data) => {

                let data_map = data.as_object().expect("Data must be an object");
                let data_str = data_map.get("value").expect("Expected 'value' field").as_str().expect("Expected 'value' to be a string");
                let data_hex = Hex::decode(data_str.as_bytes()).expect("Failed to decode hex string");
                let data_bigint = Some(BigInt { bytes: data_hex });

                assert_eq!(super::parse_value(&data_bigint), "646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d");
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn validate_data() {
        let str = "data:,{\"p\":\"asc-20\",\"op\":\"deploy\",\"tick\":\"xai\",\"max\":\"2.1e+29\",\"lim\":\"100000000000000\"}";
        let data = super::validate_data(str.to_string());

        match data {
            Ok(data) => {
                assert_eq!(data.0, "xai");  //tick
                assert_eq!(data.1, "deploy"); //op
                assert_eq!(data.2, "asc-20"); //p
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn validate_utf8() {
        let str = "0x646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d";
        let data = super::validate_utf8(&str.as_bytes().to_vec());

    match data {
        Ok(data) => {
            assert_eq!(data, true);
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
}

// Tests
// -----
// 0x646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d
// data:,{"p":"asc-20","op":"deploy","tick":"xai","max":"2.1e+29","lim":"100000000000000"}