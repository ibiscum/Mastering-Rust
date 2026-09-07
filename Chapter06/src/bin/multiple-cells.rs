use std::cell::Cell;

pub fn increment_shared_cell() -> i32 {
    let x = Cell::new(1);
    let ref_to_x_1 = &x;
    let ref_to_x_2 = &x;

    ref_to_x_1.set(ref_to_x_1.get() + 1);
    ref_to_x_2.set(ref_to_x_2.get() + 1);

    x.get()
}

fn main() {
    println!("x is now {}", increment_shared_cell());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_cell_increments_twice() {
        assert_eq!(increment_shared_cell(), 3);
    }
}
