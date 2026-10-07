use std::error::Error;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::process::exit;
use crate::exercises::util::get_args;

const FILE_NAME: &str = "budget.csv";
const SEPARATOR: &str = ";";
const FIELD_COUNT: usize = 3;
const WITHDRAW: &str = "Withdraw";
const DEPOSIT: &str = "Deposit";

#[derive(Debug)]
struct Entry {
    description: String,
    amount: u64,
    operation: Operation,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.description, SEPARATOR, self.amount, SEPARATOR, self.operation
        )
    }
}

impl TryFrom<&str> for Entry {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = value.split(SEPARATOR).collect();
        if fields.len() != FIELD_COUNT {
            return Err(AppError::ParseFailed)
        }
        let description = fields[0].to_owned();
        let amount: u64 = fields[1].parse()?;
        let operation = fields[2].try_into()?;
        let entry = Entry { description, amount, operation };
        Ok(entry)
    }
}

#[derive(Debug)]
enum Operation {
    Deposit,
    Withdraw,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Deposit => write!(f, "{DEPOSIT}"),
            Operation::Withdraw => write!(f, "{WITHDRAW}"),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            DEPOSIT => Ok(Operation::Deposit),
            WITHDRAW => Ok(Operation::Withdraw),
            _ => Err(AppError::ParseFailed)
        }
    }
}

#[derive(Debug)]
enum AppError {
    ReadFailed(io::Error),
    ParseFailed
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::ReadFailed(err) => Some(err),
            AppError::ParseFailed => None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ReadFailed(err) => write!(f, "File could not be read: {}", err),
            AppError::ParseFailed => write!(f, "Entry could not be parsed"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::ReadFailed(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> AppError {
        AppError::ParseFailed
    }
}


fn load() -> Result<Vec<Entry>, AppError> {
    let file = File::open(FILE_NAME)?;
    let reader = BufReader::new(file);
    let entries = reader
        .lines()
        .map(|line| Entry::try_from(line?.as_str()))
        .collect::<Result<Vec<_>, AppError>>()?;
    Ok(entries)
}

fn save(entries: &[Entry]) -> Result<(), AppError> {
    // truncate(true) is essential: without it a shorter rewrite leaves the
    // tail of the previous file behind and corrupts the CSV.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_NAME)?;
    for entry in entries.iter() {
        writeln!(file, "{entry}")?;
    }
    Ok(())
}
fn show_summary(entries: &[Entry]) {
    const DESC_W: usize = 30;
    const OP_W: usize = 10;
    const AMOUNT_W: usize = 12;

    // A balance must respect the operation: deposits add, withdrawals
    // subtract. It can also go negative, so u64 is the wrong type here.
    let total_balance: i64 = entries.iter()
        .map(|e| match e.operation {
            Operation::Deposit => e.amount as i64,
            Operation::Withdraw => -(e.amount as i64),
        })
        .sum();

    let line_width = DESC_W + OP_W + AMOUNT_W + 6;

    println!("{}", "-".repeat(line_width));

    println!("{:^DESC_W$} | {:^OP_W$} | {:^AMOUNT_W$}", "Description", "Operation", "Amount");

    println!("{}", "-".repeat(line_width));

    entries.iter().for_each(|e| {
        println!("{:<DESC_W$} | {:^OP_W$} | {:>AMOUNT_W$}",
            e.description,
            format!("{}", e.operation),
            e.amount
        );
    });

    println!("{}", "-".repeat(line_width));

    println!("{:<DESC_W$}   {:<OP_W$}   {:>AMOUNT_W$}", "Total Balance", "", total_balance);
}

pub fn run()  {
    let args = get_args();
    if !args.is_empty() && args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("[description amount operation]   (operation: {DEPOSIT} | {WITHDRAW})");
        exit(2); // wrong usage
    }
    let mut entries = match load() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Could not read {FILE_NAME}: {err}");
            exit(1);
        }
    };
    if args.len() == 3 {
        let entry = args.join(SEPARATOR);
        match Entry::try_from(entry.as_str()) {
            Ok(entry) => {
                entries.push(entry);
                if let Err(err) = save(&entries) {
                    eprintln!("Could not write {FILE_NAME}: {err}");
                    exit(1);
                }
            }
            // The arguments the user typed did not parse — the stored file
            // is fine, so do not blame it.
            Err(err) => {
                eprintln!("Invalid arguments: {err}");
                exit(2);
            }
        }
    }
    show_summary(&entries);
}