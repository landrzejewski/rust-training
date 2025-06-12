#[derive(Clone, PartialEq, Debug)]
enum Currency {
    Pln,
    Eur,
}

#[derive(Debug)]
struct Money {
    value: f64,
    currency: Currency,
}

impl Money {
    fn new(value: f64) -> Self {
        Self {
            value,
            currency: Currency::Pln,
        }
    }

    fn add(&mut self, money: Money) -> Option<Money> {
        if self.currency != money.currency {
            return None;
        }
        let result = Money {
            value: self.value + money.value,
            currency: self.currency.clone(),
        };
        Some(result)
    }

    fn subtract(&self, money: Money) -> Option<Money> {
        if self.currency != money.currency {
            return None;
        }
        let result = Money {
            value: self.value - money.value,
            currency: self.currency.clone(),
        };
        Some(result)
    }

    fn exchange(money: &Money, exchange_rate: f64, currency: Currency) -> Option<Money> {
        Some(Money {
            value: money.value / exchange_rate,
            currency,
        })
    }
}

pub fn run() {
    let money = Money::new(3.14);
    let result = money.subtract(Money::new(3.0)).unwrap();
    let money_in_eur = Money::exchange(&result, 3.14, Currency::Eur).unwrap();
    println!("Result: {:?}", result);
}
