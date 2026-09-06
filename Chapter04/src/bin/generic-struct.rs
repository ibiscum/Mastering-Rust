#[derive(Debug)]
#[allow(dead_code)]
struct Money<T> {
    pub(crate) amount: T,
    pub(crate) currency: String,
}

fn main() {
    let whole_euros: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let floating_euros: Money<f32> = Money { amount: 24.312, currency: "EUR".to_string() };

    println!("Whole euros: {:?}", whole_euros);
    println!("Floating euros: {:?}", floating_euros);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_can_be_created_with_different_numeric_types() {
        let whole_euros: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
        let floating_euros: Money<f32> = Money { amount: 24.312, currency: "EUR".to_string() };
        assert_eq!(whole_euros.amount, 42);
        assert_eq!(floating_euros.amount, 24.312);
    }
}
