#[derive(PartialEq, Debug, Clone)]
enum Currency {
    Pln,
    Eur
}

#[derive(Debug)]
struct Money {
    value: f64,
    currency: Currency,
}

impl Money {

    fn add(&self, other: &Money) -> Result<Money, &'static str> {
        self.check_currency(other)?;
        let new_money = Money::new(self.value + other.value, self.currency.clone());
        Ok(new_money)
    }

    fn subtract(&self, other: &Money) -> Result<Money, &'static str> {
        self.check_currency(other)?;
        let new_money = Money::new(self.value - other.value, self.currency.clone());
        Ok(new_money)
    }

    fn convert(&self, exchange_rate: f64, currency: Currency) -> Result<Money, &'static str> {
        let new_money = Money::new(self.value * exchange_rate, self.currency.clone());
        Ok(new_money)
    }

    fn check_currency(&self, other: &Money) -> Result<(), &'static str> {
        if self.currency != other.currency {
            return Err("currency mismatch");
        }
        Ok(())
    }

    fn new(value: f64, currency: Currency) -> Money {
        Money { value, currency }
    }

}

pub fn run() {
    let money = Money::new(3.14, Currency::Eur);
    let other = Money::new(60.0, Currency::Eur);
    match money.add(&other) {
        Ok(new_money) => println!("Successfully added {:?}", new_money),
        Err(msg) => println!("Error : {}", msg),
    }
}