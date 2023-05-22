pub struct Place {
    name: String
}
impl Place {
    pub fn greet(self) {
        println!("Welcome to {}", self.name)
    }
}
fn main() {
    let p = Place{ name: String::from("Rust") };
    p.greet();
}