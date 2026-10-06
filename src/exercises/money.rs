const EUR: &str = "EUR";
const PLN: &str = "PLN";

struct Money {
    value: f64,
    currency: &'static str,
}

impl Money {

    fn add(&mut self, other: &Money) -> Result<(), &'static str> {
        self.check_currency(other)?;
        self.value += other.value;
        Ok(())
    }

    fn subtract(&mut self, other: &Money) -> Result<(), &'static str> {
        self.check_currency(other)?;
        self.value -= other.value;
        Ok(())
    }

    fn convert(&mut self, exchange_rate: f64, currency: &'static str) -> Result<(), &'static str> {
        self.value *= exchange_rate;
        self.currency = currency;
        Ok(())
    }

    fn check_currency(&self, other: &Money) -> Result<(), &'static str> {
        if self.currency != other.currency {
            return Err("currency mismatch");
        }
        Ok(())
    }

    fn new(value: f64, currency: &'static str) -> Money {
        Money { value, currency }
    }

}

pub fn run() {
    let mut money = Money::new(3.14, "EUR");
    let other = Money::new(60.0, "EUR");
    match money.add(&other) {
        Ok(_) => println!("Successfully added {}", money.currency),
        Err(msg) => println!("Error : {}", msg),
    }
}