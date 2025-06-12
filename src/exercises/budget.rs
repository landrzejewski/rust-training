use std::env;
use std::fmt::{write, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

const DEPOSIT: &str = "Deposit";
const WITHDRAW: &str = "Withdraw";
const SEPARATOR: &str = ":";
const FILE_NAME: &str = "rusty_budget.txt";

enum OperationType {
    Deposit,
    Withdraw,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "{}", DEPOSIT),
            OperationType::Withdraw => write!(f, "{}", WITHDRAW),
        }
    }
}

impl TryFrom<String> for OperationType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            DEPOSIT => Ok(OperationType::Deposit),
            WITHDRAW => Ok(OperationType::Withdraw),
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

impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        static PARTS_COUNT: usize = 3;
        let parts: Vec<&str> = value.split(SEPARATOR).collect();
        if parts.len() != PARTS_COUNT {
            return Err("Invalid input".to_string());
        }
        let amount: f64 = parts[0].parse().map_err(|_| "Invalid amount")?;
        let description = parts[1].to_string();
        let operation_type: OperationType = OperationType::try_from(parts[2].to_string())?;
        let operation = Operation {
            amount,
            description,
            operation_type,
        };
        Ok(operation)
    }
}

fn load() -> Vec<Operation> {
    let file = File::open(FILE_NAME).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut operations = Vec::new();
    for line in reader.lines() {
        let text = line.expect("Could not read line");
        let operation =  Operation::try_from(text).expect("Invalid input");
        operations.push(operation);
    }
    operations
}

fn save(operations: &Vec<Operation>) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(FILE_NAME)
        .expect("Could not open file");
    operations
        .iter()
        .for_each(|operation| writeln!(file, "{operation}").expect("Could not write to file"));
}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
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

    if args.len() != 0 && args.len() < 3 {
        println!("Invalid arguments");
        exit(0);
    }
    
    let mut operations = load();

    if args.len() == 3 {
        let entry = args.join(SEPARATOR);
        let operation = Operation::try_from(entry).expect("Invalid input");
        operations.push(operation);
        save(&operations);
    }

    display_operations(&operations);
}
