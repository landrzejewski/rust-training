use std::env;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

const DEPOSIT: &str = "DEPOSIT";
const WITHDRAW: &str = "WITHDRAW";
const SEPARATOR: &str = ";";
const FIELDS_COUNT: usize = 3;
const FILE_NAME: &str = "budget.csv";

enum OperationType {
    Deposit,
    Withdraw,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OperationType::Deposit => write!(f, "{DEPOSIT}"),
            OperationType::Withdraw => write!(f, "{WITHDRAW}"),
        }
    }
}

impl TryFrom<&str> for OperationType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            DEPOSIT => Ok(OperationType::Deposit),
            WITHDRAW => Ok(OperationType::Withdraw),
            _ => Err(format!("Unknown operation type: {value}")),
        }
    }
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.amount, SEPARATOR, self.description, SEPARATOR, self.operation_type
        )
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = value.split(SEPARATOR).collect();
        if fields.len() != FIELDS_COUNT {
            return Err(format!("Invalid number of fields, expected: {value}"));
        }
        let amount: f64 = fields[0].parse().map_err(|_| "Invalid amount")?;
        let description = fields[1].to_string();
        let operation_type = OperationType::try_from(fields[2])?;
        let operation = Operation {
            amount,
            description,
            operation_type,
        };
        Ok(operation)
    }
}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn display_summary(operations: &Vec<Operation>) {
    let total_balance =
        operations
            .iter()
            .fold(0.0, |acc, operation| match operation.operation_type {
                OperationType::Deposit => acc + operation.amount,
                OperationType::Withdraw => acc - operation.amount,
            });

    operations
        .iter()
        .for_each(|operation| println!("{operation}"));
    println!("-----------------------------------------------------------");
    println!("Total amount: {total_balance}");
}

fn save(operations: &Vec<Operation>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(FILE_NAME)
        .expect("Could not open file");
    operations
        .iter()
        .for_each(|operation| writeln!(file, "{operation}").expect("Could not write to file"));
}

fn load() -> Result<Vec<Operation>, String> {
    let file = File::open(FILE_NAME).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut operations: Vec<Operation> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line from file");
        let operation = Operation::try_from(line.as_str())?;
        operations.push(operation);
    }
    Ok(operations)
}

pub fn run() {
    let args = get_args();

    if !args.is_empty() && args.len() != 3 {
        println!("Invalid number of arguments, expected 2");
        exit(0);
    }

    let mut operations = load().unwrap_or(Vec::new());

    if args.len() == 3 {
        let entry = args.join(SEPARATOR);
        match Operation::try_from(entry.as_str()) {
            Ok(operation) => {
                operations.push(operation);
                save(&operations);
            }
            Err(message) => {
                println!("{}", message);
                exit(0);
            }
        }
    }

    display_summary(&operations);
}
