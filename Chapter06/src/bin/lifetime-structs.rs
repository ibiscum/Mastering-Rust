pub struct Number<'a> {
    num: &'a u8
}

impl<'a> Number<'a> {
    pub fn new(num: &'a u8) -> Self {
        Number { num }
    }

    pub fn get_the_number(&self) -> &'a u8 {
        self.num
    }

    pub fn set_the_number(&mut self, new_number: &'a u8) {
        self.num = new_number
    }
}

fn main() {
    let inner_one = 1;
    let inner_two = 2;
    let mut num = Number::new(&inner_one);

    println!("num is now {}", num.get_the_number());

    num.set_the_number(&inner_two);

    println!("num is now {}", num.get_the_number());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_gets_and_sets() {
        let one = 1;
        let two = 2;
        let mut num = Number::new(&one);
        assert_eq!(*num.get_the_number(), 1);
        num.set_the_number(&two);
        assert_eq!(*num.get_the_number(), 2);
    }
}
