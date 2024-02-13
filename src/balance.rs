use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const FILE_NAME: &str = "operations.txt";
const DATA_SEPARATOR: &str = ";";

#[derive(Debug, PartialEq)]
enum OperationType {
    DEPOSIT,
    WITHDRAW
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

impl Operation {

    fn to_text(&self) -> String {
        String::from(format!("{}{DATA_SEPARATOR}{:?}{DATA_SEPARATOR}{}", self.amount, self.operation_type, self.description))
    }

    fn from_text(text: &String) -> Operation {
        let data: Vec<&str> = text.split(DATA_SEPARATOR).collect();

        let amount: f64 = data.get(0)
            .expect("Failed to read amount")
            .parse()
            .expect("Failed to parse amount");

        let operation_type = if amount < 0.0 { OperationType::WITHDRAW } else { OperationType::DEPOSIT };

        let description_text = data.get(1)
            .expect("Failed to read description");

        Operation{
            amount,
            operation_type,
            description: String::from(*description_text)
        }
    }
    
}

fn print_operations(operations: &Vec<Operation>) {
    operations.iter()
        .for_each(|operation| println!("{:>+10.2} zł {}", operation.amount, operation.description));
}

fn print_summary(operations: &Vec<Operation>) {
    let balance: f64 = operations.iter()
        .map(|operation| operation.amount)
        .sum();
    println!("{:-^24}", "Razem");
    println!("{:^+24.2}", balance);
}

fn save_operations(operations: &Vec<Operation>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(FILE_NAME);
    if let Ok(mut output_file) = file {
        operations.iter().for_each(|operation| writeln!(output_file, "{}", operation.to_text())
            .expect("Failed to save operation"));
    }
}

fn load_operations() -> Vec<Operation> {
    let mut operations:Vec<Operation> = Vec::new();
    let file = File::open(FILE_NAME)
        .expect("Failed to load operations");
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            operations.push(Operation::from_text(&current_line));
        }
    }
    operations
}

fn get_arguments() -> Vec<String> {
    env::args()
    .skip(1)
    .collect::<Vec<_>>()
}

fn main() {
    let mut operations = load_operations();
    let arguments = get_arguments();
    if arguments.len() == 2 {
        let entry = arguments.join(DATA_SEPARATOR);
        let operation = Operation::from_text(&entry);
        operations.push(operation);
        save_operations(&operations);
    }
    print_operations(&operations);
    print_summary(&operations);
}
