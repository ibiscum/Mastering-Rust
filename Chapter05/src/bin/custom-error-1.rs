#[derive(Debug)]
enum Currency { USD, EUR }

#[derive(Debug)]
struct CurrencyError;

impl Currency {
    fn new(currency: &str) -> Result<Self, CurrencyError> {
        match currency {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyError{})
        }
    }
}

#[derive(Debug)]
struct Money {
    currency: Currency,
    amount: u64
}

#[derive(Debug)]
#[allow(dead_code)]
struct MoneyError {
    cause: CurrencyError,
}

impl Money {
    fn new(currency: &str, amount: u64) -> Result<Self, MoneyError> {
        let currency = match Currency::new(currency) {
            Ok(c) => c,
            Err(e) => return Err(MoneyError { cause: e }),
        };

        Ok(Money {
            currency,
            amount,
        })
    }
}

fn main() {
    let money_1 = Money::new("EUR", 12345);
    let money_2 = Money::new("FIM", 600000);

    println!("Money_1 is {:?}", money_1);
    println!("Money_2 is {:?}", money_2);

    if let Ok(m) = money_1 {
        println!("Created {} with amount {}", m.currency as u8, m.amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_new_accepts_known_currencies() {
        assert!(matches!(Currency::new("USD"), Ok(Currency::USD)));
        assert!(matches!(Currency::new("EUR"), Ok(Currency::EUR)));
    }

    #[test]
    fn currency_new_rejects_unknown_currency() {
        assert!(Currency::new("FIM").is_err());
    }

    #[test]
    fn money_new_succeeds_for_valid_currency() {
        let money = Money::new("EUR", 12345).unwrap();
        assert!(matches!(money.currency, Currency::EUR));
        assert_eq!(money.amount, 12345);
    }

    #[test]
    fn money_new_returns_error_for_invalid_currency() {
        assert!(Money::new("FIM", 600000).is_err());
    }
}
