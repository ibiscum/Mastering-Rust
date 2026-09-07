fn take_the_n(n: &u8) {
    println!("The n is {}", n);
}

fn take_the_s(s: &String) {
    println!("The s is {}", s);
}

fn main() {
    let n = 5;
    let s = String::from("string");

    take_the_n(&n);
    take_the_s(&s);

    println!("n is {}", n);
    println!("s is {}", s);
}
