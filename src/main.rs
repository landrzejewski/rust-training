use std::iter::Iterator;

const FIELD_SIZES: [usize; 5] = [8, 10, 20, 1, 1];
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
    String::from_utf8(bytes).expect("Found invalid UTF-8")
        .chars()
        .filter(|c| *c != EMPTY_VALUE)
        .collect()
}

#[derive(Debug)]
struct User {
    id: i64,
    first_name: String,
    last_name: String,
    is_active: bool,
    age: u8
}

fn user_to_record(user: &User) -> Vec<u8> {
    let first_name_len = user.first_name.len();
    if first_name_len > FIELD_SIZES[1] {
        panic!("First name is too big")
    }
    if user.last_name.len() > FIELD_SIZES[2] {
        panic!("Last name is too big")
    }
    [
        i64_to_bytes(user.id),
        string_to_bytes(&user.first_name, FIELD_SIZES[1]),
        string_to_bytes(&user.last_name, FIELD_SIZES[2]),
        bool_to_bytes(user.is_active),
        u8_to_bytes(user.age)
    ].concat()
}

fn record_to_user(bytes: &Vec<u8>) -> User {
    User {
        id: i64_from_bytes(bytes[0..8].to_vec()),
        first_name: string_from_bytes(bytes[8..18].to_vec()),
        last_name: string_from_bytes(bytes[18..38].to_vec()),
        is_active: bool_from_bytes([bytes[38]]),
        age: u8_from_bytes([bytes[39]]),
    }
}

fn main() {
    let user = User {
        id: 1,
        first_name: String::from("Jan"),
        last_name: String::from("Kowalski"),
        is_active: true,
        age: 18
    };

    let bytes = user_to_record(&user);
    println!("{:#?}", record_to_user(&bytes));
}