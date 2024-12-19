const EUR: &str = "EUR";
const PLN: &str = "PLN";

#[derive(Debug)]
struct MonetaryAmount {
    value: f64,
    currency: &'static str,
}

impl MonetaryAmount {
    fn add(&mut self, other: &MonetaryAmount) -> Result<f64, String> {
        self.check_currency(other)?;
        self.value += other.value;
        Ok(self.value)
    }

    fn subtract(&mut self, other: &MonetaryAmount) -> Result<f64, String> {
        self.check_currency(other)?;
        self.value -= other.value;
        Ok(self.value)
    }

    fn check_currency(&self, other: &MonetaryAmount) -> Result<(), String> {
        if self.currency != other.currency {
            return Err(String::from("Invalid currency"));
        }
        Ok(())
    }

    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: &'static str) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency,
        }
    }
}

pub fn run() {
    let mut balance = MonetaryAmount { value: 1_000.0, currency: EUR };

    let result = balance.add(&MonetaryAmount { value: 10.0, currency: EUR });

    println!("Operation result: {result:?}");
    println!("Balance: {:?}", balance);
    let pln_balance = MonetaryAmount::convert(&balance, 0.2, PLN);
    println!("Balance: {:?}", pln_balance);
}
