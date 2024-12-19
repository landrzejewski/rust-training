use std::env::args;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

const DESCRIPTION_SEPARATOR: &str = " ";
const FIELD_SEPARATOR: &str = ":";
const FILE_NAME: &str = "rusty_budget.txt";

enum OperationType {
    Deposit,
    Withdraw,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "Deposit"),
            OperationType::Withdraw => write!(f, "Withdraw"),
        }
    }
}

impl TryFrom<String> for OperationType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "Deposit" => Ok(OperationType::Deposit),
            "Withdraw" => Ok(OperationType::Withdraw),
            _ => Err(format!("Unknown operation type: {}", value)),
        }
    }
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.amount, self.description, self.operation_type
        )
    }
}

fn get_args() -> Vec<String> {
    args().skip(1).collect()
}

fn parse(args: &Vec<String>) -> Result<Operation, String> {
    if args.len() < 2 {
        return Err(String::from("Not enough arguments"));
    }
    let Some(amount_text) = args.first() else {
        return Err(String::from("Not enough arguments"));
    };
    let Ok(amount) = amount_text.parse::<f64>() else {
        return Err(String::from("Invalid amount value"));
    };
    let operation_type = if amount < 0.0 {
        OperationType::Withdraw
    } else {
        OperationType::Deposit
    };
    let amount = amount.abs();
    let description = args
        .iter()
        .skip(1)
        .map(String::as_str)
        .collect::<Vec<&str>>()
        .join(DESCRIPTION_SEPARATOR);
    let operation = Operation {
        amount,
        description,
        operation_type,
    };
    Ok(operation)
}

fn load_operations() -> Vec<Operation> {
    let file = File::open(FILE_NAME).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut operations: Vec<Operation> = Vec::new();
    for line in reader.lines() {
        let fields = line
            .expect("Could not read line")
            .split(FIELD_SEPARATOR)
            .map(|field| field.into())
            .collect::<Vec<String>>();
        let operation = Operation {
            amount: get_field(0, &fields).parse().expect("Invalid amount"),
            description: get_field(1, &fields),
            operation_type: get_field(2, &fields)
                .try_into()
                .expect("Invalid operation type"),
        };
        operations.push(operation);
    }
    operations
}

fn get_field(index: usize, fields: &Vec<String>) -> String {
    let Some(field) = fields.get(index) else {
        return "".into();
    };
    field.clone()
}

fn save_operations(operations: &Vec<Operation>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(FILE_NAME)
        .expect("Couldn't open file");
    operations
        .iter()
        .for_each(|operation| writeln!(file, "{}", operation).expect("Couldn't write to file"));
}

fn display_operations(operations: &Vec<Operation>) {
    let total_amount =
        operations
            .iter()
            .fold(0.0, |balance, operation| match operation.operation_type {
                OperationType::Deposit => balance + operation.amount,
                OperationType::Withdraw => balance - operation.amount,
            });
    operations
        .iter()
        .for_each(|operation| println!("{}", operation));
    println!("-------------------------------------------------------------------");
    println!("Total amount: {:.2}", total_amount);
}

pub fn run() {
    let args = get_args();
    if args.len() != 0 && args.len() < 2 {
        println!("Invalid arguments");
        exit(0);
    }
    let mut operations = load_operations();
    if let Ok(operation) = parse(&args) {
        operations.push(operation);
        save_operations(&operations);
    }
    display_operations(&operations);
}
