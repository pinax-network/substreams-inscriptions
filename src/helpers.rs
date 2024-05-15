use std::str;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::BigInt;

pub fn json_to_i64(value: &serde_json::Value, index: &str) -> Option<i64> {
    value
        .get(index)
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i64>().ok())
}

pub fn json_to_string(value: &serde_json::Value, index: &str) -> String {
    value
        .get(index)
        .and_then(|v| v.as_str())
        .map_or(String::new(), |s| s.to_string())
}

pub fn parse_value(value: &BigInt) -> String {
    let encoded = Hex(&value.bytes).to_string();
    if encoded.is_empty() {
        String::from("0")
    } else {
        encoded
    }
}

pub fn parse_data(utf8: &str) -> Option<&str> {
    utf8.strip_prefix("data:")
        .and_then(|s| s.split_once(',').map(|(_, rest)| rest))
}

pub fn _hex_to_utf8(hex: &str) -> Option<String> {
    Hex::decode(hex)
        .ok()
        .and_then(|bytes| bytes_to_utf8(&bytes))
}

pub fn bytes_to_utf8(bytes: &[u8]) -> Option<String> {
    str::from_utf8(bytes).ok().map(|s| s.to_string())
}

pub fn parse_mime_type(utf8: &str) -> Option<&str> {
    let parts = utf8.split_once("data:").map(|x| x.1);
    match parts {
        Some(parts) => {
            // split "application/json,inscription" into "application/json" and "inscription"
            let subparts = parts.split(',').next();
            match subparts {
                Some(subparts) => {
                    // if no subparts "data:,inscription" then return "text/plain"
                    if subparts.is_empty() {
                        return Some("text/plain");
                    }
                    Some(subparts)
                }
                None => None,
            }
        }
        None => None,
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
            }
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
            }
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
            }
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
                let data_str = data_map
                    .get("value")
                    .expect("Expected 'value' field")
                    .as_str()
                    .expect("Expected 'value' to be a string");
                let data_hex =
                    Hex::decode(data_str.as_bytes()).expect("Failed to decode hex string");
                let data_bigint = BigInt { bytes: data_hex };

                assert_eq!(super::parse_value(&data_bigint), "646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d");
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn parse_data_to_json_string() {
        let str = "data:,{\"p\":\"asc-20\",\"op\":\"deploy\",\"tick\":\"xai\",\"max\":\"2.1e+29\",\"lim\":\"100000000000000\"}";
        let data = super::parse_data(str);

        match serde_json::from_str(&data.unwrap()) {
            Ok(json_data) => {
                let tick = super::json_to_string(&json_data, "tick");
                let op = super::json_to_string(&json_data, "op");
                let p = super::json_to_string(&json_data, "p");
                assert_eq!(tick, "xai"); //tick
                assert_eq!(op, "deploy"); //op
                assert_eq!(p, "asc-20"); //p
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn parse_data() {
        // valid
        assert_eq!(super::parse_data(&super::_hex_to_utf8("646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d").unwrap()).is_none(), false);
        assert_eq!(super::parse_data(&super::_hex_to_utf8("646174613a6170706c69636174696f6e2f6a736f6e2c7b2270223a22696572632d3230222c226f70223a227472616e73666572222c227469636b223a2269657263222c226e6f6e6365223a2231373134383139343436323630222c22746f223a5b7b22616d74223a223230303030222c2272656376223a22307837303434373445313831433231384237363836423062624233643839433632444437653338326531227d5d7d").unwrap()).is_none(), false);
        assert_eq!(super::parse_data(&super::_hex_to_utf8("646174613a2c7b2270223a226173632d3230222c226f70223a227472616e73666572222c22616d74223a223631363336353635373637222c227469636b223a2261766176227d").unwrap()).is_none(), false);

        // invalid
        assert_eq!(super::parse_data(&super::_hex_to_utf8("58666c04000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000697b2270726f746f636f6c223a2022616c6570682d6f6666636861696e222c202276657273696f6e223a20312c2022636f6e74656e74223a2022516d5356627268735378416337635046653345334c584759384636487644474e313957426875645574466d63736d227d0000000000000000000000000000000000000000000000").unwrap()).is_none(), true);
        assert_eq!(super::parse_data(&super::_hex_to_utf8("48692c63616e20796f752067697665206d6520736f6d6520736869626120746f6b656e2c697420776f756c64206265206d7920677265617420706c6561737572652e").unwrap()).is_none(), true);
        assert_eq!(super::parse_data(&super::_hex_to_utf8("6920616d207665727920706f6f7220616e6420616666656374656420627920636f7669642c20706c656173652068656c70206d65").unwrap()).is_none(), true);
    }

    #[test]
    fn bytes_to_utf8() {
        assert_eq!(
            super::bytes_to_utf8(&vec![0x64, 0x61, 0x74, 0x61]).unwrap(),
            "data"
        );
    }

    #[test]
    fn parse_mime_type() {
        // valid
        assert_eq!(
            super::parse_mime_type("data:application/json").unwrap(),
            "application/json"
        );
        assert_eq!(
            super::parse_mime_type("data:text/plain").unwrap(),
            "text/plain"
        );
        assert_eq!(super::parse_mime_type("data:").unwrap(), "text/plain");
        assert_eq!(
            super::parse_mime_type("data:,inscription").unwrap(),
            "text/plain"
        );
        assert_eq!(
            super::parse_mime_type("data:application/vnd.facet.tx+json;rule=esip6,{\"data\":")
                .unwrap(),
            "application/vnd.facet.tx+json;rule=esip6"
        );

        // invalid
        assert_eq!(super::parse_mime_type("foobar").is_none(), true);
        assert_eq!(super::parse_mime_type("").is_none(), true);
    }
}

// Tests
// -----
// 0x646174613a2c7b2270223a226173632d3230222c226f70223a226465706c6f79222c227469636b223a22786169222c226d6178223a22322e31652b3239222c226c696d223a22313030303030303030303030303030227d
// data:,{"p":"asc-20","op":"deploy","tick":"xai","max":"2.1e+29","lim":"100000000000000"}
