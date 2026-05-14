const EUR: &str = "EUR";
const PLN: &str = "PLN";

#[derive(Debug)]
struct MonetaryAmount {
    value: f64,
    currency: String,
}

impl MonetaryAmount {
    fn add(&mut self, other: &MonetaryAmount) -> Result<(), String> {
        self.check_currency(other)?;
        self.value += other.value;
        Ok(())
    }

    fn subtract(&mut self, other: &MonetaryAmount) -> Result<(), String> {
        self.check_currency(other)?;
        self.value -= other.value;
        Ok(())
    }

    fn check_currency(&self, other: &MonetaryAmount) -> Result<(), String> {
        if self.currency != other.currency {
            return Err(String::from("Currency not match!"));
        }
        Ok(())
    }

    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: &str) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency: String::from(currency),
        }
    }

    fn new(value: f64, currency: &str) -> Self {
        Self {
            value,
            currency: String::from(currency),
        }
    }
}

pub fn run() {
    let mut balance = MonetaryAmount::new(1_000.0, EUR);
    let income = MonetaryAmount::new(2_000.0, EUR);

    match balance.add(&income) {
        Ok(_) => println!("Balance updated: {:?}", balance),
        Err(message) => println!("Error: {message}"),
    }

    let result_pln = MonetaryAmount::convert(&balance, 0.2, PLN);
    println!("Balance converted: {:?}", result_pln);
}
