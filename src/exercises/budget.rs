use std::env;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const FILE_NAME: &str = "rusty_budget";
const FIELD_SEPARATOR: &str = ";";

enum OperationType {
    DEPOSIT,
    WITHDRAW
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

impl Display for Operation {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.amount, FIELD_SEPARATOR, self.description)
    }

}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn parse_operation(args: &Vec<String>) -> Result<Operation, &str> {
    /*
    let Some(amount) = args.get(0).and_then(|text| text.trim().parse::<f64>().ok()) else {
        return Err("Invalid amount value");
    };
    */

    let Some(amount_text) = args.get(0) else {
        return Err("Amount not provided")
    };
    let Ok(amount) = amount_text.trim().parse::<f64>() else {
        return Err("Amount is not a valid number");
    };

    let Some(description) = args.get(1) else {
        return Err("Description not provided")
    };

    let operation_type = if amount >= 0.0 {
        OperationType::DEPOSIT
    } else {
        OperationType::WITHDRAW
    };
    Ok(Operation {
        amount,
        description: description.to_string(),
        operation_type
    })
}

fn load_operations() -> Vec<Operation> {
    let file = File::open(FILE_NAME).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut operations: Vec<Operation> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let fields = line
            .expect("Couldn't read line")
            .split(FIELD_SEPARATOR)
            .map(|field| field.to_string())
            .collect::<Vec<String>>();
        let operation= parse_operation(&fields)
            .expect("Couldn't parse operation");
        operations.push(operation);
    }
    operations
}

fn save_operations(operations: &Vec<Operation>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(FILE_NAME)
        .expect("Couldn't open file");
    operations.iter()
        .for_each(|operation|
            writeln!(file, "{}", operation).expect("Couldn't write to file")
        );
}

pub fn run() -> Result<(), &'static str> {
    let args = get_args();
    if args.len() == 2 {
        let Ok(operation) = parse_operation(&args) else {
            return Err("Parsing arguments failed");
        };

    }
    Ok(())
}