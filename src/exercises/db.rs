const EMPTY_VALUE: char = '\0';

fn i64_to_bytes(value: i64) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

fn i64_from_bytes(bytes: Vec<u8>) -> i64 {
    let mut bytes_arr = [0; 8];
    bytes_arr.clone_from_slice(&bytes[0..8]);
    i64::from_be_bytes(bytes_arr)
}

fn u8_to_bytes(value: u8) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

fn u8_from_bytes(bytes: [u8; 1]) -> u8 {
    u8::from_be_bytes(bytes)
}

fn bool_to_bytes(value: bool) -> Vec<u8> {
    if value { vec![1u8] } else { vec![0u8] }
}

fn bool_from_bytes(bytes: [u8; 1]) -> bool {
    if bytes == [1u8] { true } else { false }
}

fn string_to_bytes(value: &String, size: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![EMPTY_VALUE as u8; size];
    for (index, byte) in value.clone().into_bytes().iter().enumerate() {
        bytes[index] = *byte
    }
    bytes
}

fn string_from_bytes(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes)
        .expect("Found invalid UTF-8")
        .chars()
        .filter(|c| *c != EMPTY_VALUE)
        .collect()
}