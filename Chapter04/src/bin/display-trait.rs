use std::fmt::{Formatter, Display, Result};

struct Money<T> {
    pub(crate) amount: T,
    pub(crate) currency: String,
}

impl<T: Display> Display for Money<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

fn main() {
    let money = Money { amount: 42, currency: "EUR".to_string() };
    println!("Displaying money: {}", money);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_displays_amount_and_currency() {
        let money = Money { amount: 42, currency: "EUR".to_string() };
        assert_eq!(format!("{}", money), "42 EUR");
    }
}



