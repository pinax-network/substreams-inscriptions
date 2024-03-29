pub fn balance_key(p: String, tick: String, address: String) -> String {
    format!("{}-{}-{}", p, tick, address)
}