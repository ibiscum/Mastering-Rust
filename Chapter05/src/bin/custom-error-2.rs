use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
enum Currency {
    USD,
    EUR
}


#[derive(Debug)]
struct CurrencyError {
    description: String
}
impl Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CurrencyError: {}", self.description)
    }
}

impl Error for CurrencyError {
    fn description(&self) -> &str {
        "CurrencyError"
    }
}

impl Currency {
    fn new(currency: &str) -> Result<Self, CurrencyError> {
        match currency {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyError{ description: format!("{} not a valid currency", currency)})
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Money {
    currency: Currency,
    amount: u64,
}

#[derive(Debug)]
struct MoneyError {
    cause: CurrencyError
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoneyError due to {}", self.cause)
    }
}

impl Error for MoneyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

impl Money {
    fn new(currency: &str, amount: u64) -> Result<Self, MoneyError> {
        let currency = match Currency::new(currency) {
            Ok(c) => c,
            Err(e) => return Err(MoneyError { cause: e })
        };

        Ok(Money {
            currency: currency,
            amount: amount
        })
    }
}

fn main() {
    let money_1 = Money::new("EUR", 12345);
    let money_2 = Money::new("FIM", 600000);

    println!("Money_1 is {:?}", money_1);
    println!("Money_2 is {:?}", money_2);

    let cause_for_money_2 = money_2.unwrap_err();
    println!("{}", cause_for_money_2);
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
        let err = Currency::new("FIM").unwrap_err();
        assert_eq!(err.description, "FIM not a valid currency");
    }

    #[test]
    fn money_new_succeeds_for_valid_currency() {
        let money = Money::new("EUR", 12345).unwrap();
        assert!(matches!(money.currency, Currency::EUR));
        assert_eq!(money.amount, 12345);
    }

    #[test]
    fn money_new_propagates_currency_error() {
        let err = Money::new("FIM", 600000).unwrap_err();
        assert_eq!(err.cause.description, "FIM not a valid currency");
    }

    #[test]
    fn money_error_display_includes_cause() {
        let err = Money::new("FIM", 600000).unwrap_err();
        let text = format!("{}", err);
        assert!(text.contains("FIM not a valid currency"));
    }
}
