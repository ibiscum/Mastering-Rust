#[derive(Debug)]
#[allow(dead_code)]
struct Money<T> {
    pub(crate) amount: T,
    pub(crate) currency: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CurrencylessMoney<T> {
    pub(crate) amount: T,
}

impl<T> From<Money<T>> for CurrencylessMoney<T> {
    fn from(money: Money<T>) -> CurrencylessMoney<T> {
        CurrencylessMoney { amount: money.amount }
    }
}

fn main() {
    let money = Money { amount: 42, currency: "EUR".to_string() };
    let currencyless_money: CurrencylessMoney<u32> = money.into();

    println!("Money without currency: {:?}", currencyless_money);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_converts_into_currencyless_money() {
        let money = Money { amount: 42, currency: "EUR".to_string() };
        let currencyless: CurrencylessMoney<u32> = money.into();
        assert_eq!(currencyless.amount, 42);
    }
}
