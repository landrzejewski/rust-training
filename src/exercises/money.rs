const EUR: &str = "EUR";
const PLN: &str = "PLN";

#[derive(Debug)]
struct MonetaryAmount {
    value: f64,
    currency: &'static str,
}

impl MonetaryAmount {
    fn add(&mut self, other: &MonetaryAmount) {
        self.check_currency(other);
        self.value += other.value;
    }

    fn subtract(&mut self, other: &MonetaryAmount) {
        self.check_currency(other);
        self.value -= other.value;
    }

    fn check_currency(&self, other: &MonetaryAmount) {
        if self.currency != other.currency {
            panic!("Invalid currency");
        }
    }

    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: &'static str) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency,
        }
    }
}

pub fn run() {
    let mut balance = MonetaryAmount {
        value: 1_000.0,
        currency: EUR,
    };

    balance.add(&MonetaryAmount {
        value: 10.0,
        currency: EUR,
    });

    println!("Balance: {:?}", balance);

    let pln_balance = MonetaryAmount::convert(&balance, 0.2, PLN);

    println!("Balance: {:?}", pln_balance);
}
