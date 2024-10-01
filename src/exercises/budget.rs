use std::env;

enum OperationType {
    DEPOSIT,
    WITHDRAW
}

struct Operation {
    amount: f64,
    description: String,
    operation_type: OperationType,
}

fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn parse_operation(args: &Vec<String>) -> Result<Operation, &str> {
    /*let Some(amount) = args
        .get(0)
        .and_then(|text| text.trim().parse::<f64>().ok()) else {
        return Err("Invalid amount value");
    };*/

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

pub fn run() -> Result<(), &'static str> {
    let args = get_args();
    if args.len() != 2 {
        return Err("Wrong number of arguments");
    }
    let Ok(operation) = parse_operation(&args) else {
        return Err("Parse args failed");
    };

    Ok(())
}