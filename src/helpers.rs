use std::str;

use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BigInt;

pub fn json_to_i64(value: &serde_json::Value, index: &str) -> Option<i64> {
    if let Some(value) = value.get(index) {
        if let Some(str) = value.as_str() {
            Some(str.parse::<i64>().unwrap())
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

#[cfg(test)]
mod tests {
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
}