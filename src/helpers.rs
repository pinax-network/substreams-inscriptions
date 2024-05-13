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

pub fn parse_json_data(input: &String) -> Result<Value, serde_json::Error> {
    let json_str = input.splitn(2, ',').nth(1).unwrap_or_default();
    return serde_json::from_str(json_str)
}

pub fn parse_input(data: &Vec<u8>) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    match str::from_utf8(&data) {
        Ok(vec) => vec.to_string(),
        Err(_e) => "".to_string(),
    }
}

pub fn parse_mime_type(input: &String) -> String {
    if input.starts_with("data:application/json") {
        return "application/json".to_string();
    }
    else if input.starts_with("data:text/plain") {
        return "text/plain".to_string();
    }
    else if input.starts_with("data") {
        return "text/plain".to_string();
    }
    else {
        return "".to_string();
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
    fn parse_json_data() {
        let str = "data:,{\"p\":\"asc-20\",\"op\":\"deploy\",\"tick\":\"xai\",\"max\":\"2.1e+29\",\"lim\":\"100000000000000\"}";
        let data = super::parse_json_data(&str.to_string());

        match data {
            Ok(json_data) => {
                let tick = super::json_to_string(&json_data, "tick");
                let op = super::json_to_string(&json_data, "op");
                let p = super::json_to_string(&json_data, "p");
                assert_eq!(tick, "xai");  //tick
                assert_eq!(op, "deploy"); //op
                assert_eq!(p, "asc-20"); //p
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn parse_input() {
        let data = vec![0x64, 0x61, 0x74, 0x61]; // data
        assert_eq!(super::parse_input(&data), "data");
    }

    #[test]
    fn parse_mime_type() {
        assert_eq!(super::parse_mime_type(&"data:application/json".to_string()), "application/json");
        assert_eq!(super::parse_mime_type(&"data:text/plain".to_string()), "text/plain");
        assert_eq!(super::parse_mime_type(&"data:".to_string()), "text/plain");
        assert_eq!(super::parse_mime_type(&"foobar".to_string()), "");
        assert_eq!(super::parse_mime_type(&"".to_string()), "");
    }
}


// Tests
// -----
// 0x646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d
// data:,{"p":"asc-20","op":"deploy","tick":"xai","max":"2.1e+29","lim":"100000000000000"}