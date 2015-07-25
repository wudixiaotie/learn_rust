fn main() {
    let a = 6u8;
    let b = "Hello".to_string();

    println!("{}", do_something(a));
    println!("{}", do_something(b));
    println!("Hello, world!");
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_something<T: Foo>(x: T) -> String {
    x.method()
}