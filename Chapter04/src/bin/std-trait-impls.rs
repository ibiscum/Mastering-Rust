use std::ops::Add;

#[derive(Debug)]
struct Money<T> {
    pub(crate) amount: T,
    pub(crate) currency: String,
}

impl<T: Add<T, Output=T>> Add for Money<T> {
    type Output = Money<T>;
    fn add(self, rhs: Money<T>) -> Self::Output {
        assert!(self.currency == rhs.currency);
        Money { currency: rhs.currency, amount: self.amount + rhs.amount }
    }
}

fn main() {
    let whole_euros_1: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let whole_euros_2: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let summed_euros = whole_euros_1 + whole_euros_2;

    println!("Summed euros: {:?}", summed_euros);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_addition_sums_amounts_when_currencies_match() {
        let m1 = Money { amount: 42, currency: "EUR".to_string() };
        let m2 = Money { amount: 42, currency: "EUR".to_string() };
        let sum = m1 + m2;
        assert_eq!(sum.amount, 84);
        assert_eq!(sum.currency, "EUR");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn money_addition_panics_on_currency_mismatch() {
        let m1 = Money { amount: 42, currency: "EUR".to_string() };
        let m2 = Money { amount: 42, currency: "USD".to_string() };
        let _sum = m1 + m2;
    }
}
