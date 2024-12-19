use std::collections::HashMap;

pub fn run() {
    vec![
        ("Electronics", 1000),
        ("Furniture", 500),
        ("Electronics", 1200),
        ("Furniture", 800),
        ("Clothing", 200),
        ("Clothing", 300),
        ("Electronics", 1100),
        ("Clothing", 400),
    ]
    .into_iter()
    .fold(HashMap::new(), |mut acc, (category, sales)| {
        acc.entry(category).or_insert(vec![]).push(sales);
        acc
    })
    .iter()
    .for_each(|(category, sales)| print_summary(*category, sales));
}

fn print_summary(category: &str, rows_in_category: &Vec<i32>) {
    let min = rows_in_category.iter().min().unwrap();
    let max = rows_in_category.iter().max().unwrap();

    let count = rows_in_category.len();
    let sum: i32 = rows_in_category.iter().sum();
    let avg = sum as f32 / count as f32;
    println!("Category: {:<12} | Avg: {:<8.2} | Min: {:<6} | Max: {:<6}", category, avg, min, max);
}
