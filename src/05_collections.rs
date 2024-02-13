use std::collections::HashMap;

fn main() {
    // Vector

    let mut numbers:Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    let mut numbers = vec![1, 2, 3];
    let selected_number = numbers[0];
    // numbers[4]; // panic - index out of range

    let mut num_vec: Vec<i32> = Vec::with_capacity(8);
    let my_vec: Vec<u8> = [1, 2, 3].into(); // konwersja z tablicy

    if let Some(number) = numbers.get(2) {
        println!("Value with index 2: {number}");
    }

    // slices

    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];
    println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}", three_to_five, start_at_two, end_at_five, everything);

    for number in &mut numbers {
        println!("Number: {number}");
    }

    let mut itearator = numbers.iter();
    // let number = itearator.next();

    let result: Vec<i32> = itearator
        .map(|number| number + 1)
        .filter(|number| number % 2 == 0)
        .collect();

    // String

    let text = String::from("abc"); // zmienna text jest "dynamically sized", wskazuje na ciąg znaków na stercie (pointer + length), jest właścicielem danych, pozwala na mutację
    let my_string: String = "Try to make this a String".into();
    let text_slice = "abc";  // długość znana w czasie kompilacji, dostępne w czasie działani aplikacji (dodawana do pliku binarnego)

    let text = "Łukasz".to_string(); // String::from("Łukasz");
    for letter in text.chars() {
        println!("Letter: {letter}");
    }
    let first_letter = text.chars().nth(0); // bezpieczny dostęp
    let text_length = text.len();

    let text_slice = &text[0..2];
    println!("Text slice: {text_slice}");

    let name = "John";
    let country = "PL";
    let home = "Warsaw";
    let together = format!("I am {name} and I come from {country} but I live in {home}.");

    // HashMap (BTreeMap zapewnia kolejność kluczy)

    let mut ratings: HashMap<&str, i32> = HashMap::new();
    ratings.insert("a", 10);
    ratings.insert("b", 11);
    let rate = ratings.entry("a").or_insert(12);

    if let Some(rate) = ratings.get("a") {
        println!("Rate: {rate}");
    }

    for (key, rate) in &ratings {
        println!("Rate: {}:{rate}", *key);
    }

    // HashSet and BTreeSet, VecDeque

}
