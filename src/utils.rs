pub fn to_hex_string(bytes: &[u8]) -> String {
    let strings: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strings.join("")
}
