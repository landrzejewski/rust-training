use std::ops::Add;

#[derive(PartialEq, Debug, Clone)]
enum Currency {
    Pln,
    Eur,
}

#[derive(Debug)]
struct MonetaryAmount {
    value: f64,
    currency: Currency,
}

impl MonetaryAmount {
    fn addValue(&self, other: &MonetaryAmount) -> Result<MonetaryAmount, String> {
        self.check_currency(other)?;
        Ok(Self {
            value: self.value + other.value,
            currency: self.currency.clone(),
        })
    }

    fn subtractValue(&self, other: &MonetaryAmount) -> Result<MonetaryAmount, String> {
        self.check_currency(other)?;
        Ok(Self {
            value: self.value - other.value,
            currency: self.currency.clone(),
        })
    }

    fn check_currency(&self, other: &MonetaryAmount) -> Result<&MonetaryAmount, String> {
        if self.currency != other.currency {
            return Err(String::from("Currency not match!"));
        }
        Ok(self)
    }

    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: &Currency) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency: currency.clone(),
        }
    }

    fn new(value: f64, currency: &Currency) -> Self {
        Self {
            value,
            currency: currency.clone(),
        }
    }
}

impl Add for MonetaryAmount {

    type Output = Result<MonetaryAmount, String>;

    fn add(self, rhs: Self) -> Self::Output {
        self.addValue(&rhs)
    }

}

pub fn run() {
    let mut balance = MonetaryAmount::new(1_000.0, &Currency::Eur);
    let income = MonetaryAmount::new(2_000.0, &Currency::Eur);

    // let a = balance + income;

    match balance.addValue(&income) {
        Ok(_) => println!("Balance updated: {:?}", balance),
        Err(message) => println!("Error: {message}"),
    }

    let result_pln = MonetaryAmount::convert(&balance, 0.2, &Currency::Pln);
    println!("Balance converted: {:?}", result_pln);
}
