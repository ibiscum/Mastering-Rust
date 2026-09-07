struct Character {
    name: String
}

impl Drop for Character {
    fn drop(&mut self) {
        println!("{} went away", self.name)
    }
}

fn main() {
    let _steve = Character { name: "Steve".into() };
    let _john = Character { name: "John".into() };
}
