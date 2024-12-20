use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::iter::Iterator;

// -------------- Database --------------
const DATABASE_FILE: &str = "users.dat";
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

trait Serializable {

    fn serialize(&self) -> Vec<u8>;

    fn deserialize(bytes: &Vec<u8>) -> Self;

    fn get_field_sizes() -> Vec<usize>;

}

fn write_record<T: Serializable>(data: &T) {
    let bytes = data.serialize();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DATABASE_FILE)
        .unwrap();
    file.seek(SeekFrom::End(0)).unwrap();
    file.write(bytes.as_slice()).unwrap();
}

fn read_record<T: Serializable>(index: u64) -> T {
    let mut file = OpenOptions::new()
        .read(true)
        .open(DATABASE_FILE)
        .unwrap();
    let record_size = T::get_field_sizes().iter().sum();
    let position = record_size as u64 * index;
    file.seek(SeekFrom::Start(position)).unwrap();
    let mut bytes = vec![0; record_size];
    file.read_exact(&mut bytes).unwrap();
    T::deserialize(&bytes)
}

// --------------- Users ----------------

#[derive(Debug)]
struct User {
    id: i64,
    first_name: String,
    last_name: String,
    is_active: bool,
    age: u8,
}

impl Serializable for User {
    fn serialize(&self) -> Vec<u8> {
        let filed_sizes = Self::get_field_sizes();
        if self.first_name.len() > filed_sizes[1] {
            panic!("First name is too big")
        }
        if self.last_name.len() > filed_sizes[2] {
            panic!("Last name is too big")
        }
        [
            i64_to_bytes(self.id),
            string_to_bytes(&self.first_name, filed_sizes[1]),
            string_to_bytes(&self.last_name, filed_sizes[2]),
            bool_to_bytes(self.is_active),
            u8_to_bytes(self.age),
        ]
            .concat()
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        User {
            id: i64_from_bytes(bytes[0..8].to_vec()),
            first_name: string_from_bytes(bytes[8..18].to_vec()),
            last_name: string_from_bytes(bytes[18..38].to_vec()),
            is_active: bool_from_bytes([bytes[38]]),
            age: u8_from_bytes([bytes[39]]),
        }
    }

    fn get_field_sizes() -> Vec<usize> {
        vec![8, 10, 20, 1, 1]
    }

}

pub fn run() {
    let first_user = User {
        id: 1,
        first_name: String::from("Łukasz"),
        last_name: String::from("Kowalski"),
        is_active: false,
        age: 18,
    };
    let second_user = User {
        id: 2,
        first_name: String::from("Marek"),
        last_name: String::from("Nowak"),
        is_active: true,
        age: 23,
    };

    write_record(&first_user);
    write_record(&second_user);

    let read_first_user: User = read_record(0);
    println!("{:#?}", read_first_user);
    let read_second_user: User = read_record(1);
    println!("{:#?}", read_second_user);
}
