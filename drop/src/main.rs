fn main() {
    let x = A;
}

struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}