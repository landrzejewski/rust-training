#[derive(Copy, Clone, Debug, PartialEq)]
enum Currency {
    EUR,
    PLN,
}

#[derive(Debug)]
struct MonetaryAmount {
    value: f64,
    currency: Currency,
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

    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: Currency) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency,
        }
    }
}

pub fn run() {
    let mut balance = MonetaryAmount {
        value: 1_000.0,
        currency: Currency::PLN,
    };

    balance.add(&MonetaryAmount {
        value: 10.0,
        currency: Currency::PLN,
    });

    println!("Balance: {:?}", balance);

    let pln_balance = MonetaryAmount::convert(&balance, 0.2, Currency::PLN);

    println!("Balance: {:?}", pln_balance);
}
